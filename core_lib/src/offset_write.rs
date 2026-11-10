use std::fs::File;
use std::io;

/// Write the entire buffer at an explicit byte offset.
///
/// Unix preserves the cursor. Windows `seek_write` changes it, as documented by
/// std; callers must serialize cursor-based access to the same file. Inbound
/// transfers already process each file's chunks sequentially at explicit offsets.
pub fn write_all_at_offset(file: &File, data: &[u8], offset: u64) -> io::Result<()> {
    offset
        .checked_add(data.len() as u64)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "file write range overflows"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::FileExt;
        file.write_all_at(data, offset)
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::FileExt;
        write_all_with(data, offset, |buffer, position| {
            file.seek_write(buffer, position)
        })
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (file, data, offset);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "offset writes require Unix or Windows",
        ))
    }
}

#[cfg(any(windows, test))]
fn write_all_with(
    mut data: &[u8],
    mut offset: u64,
    mut write: impl FnMut(&[u8], u64) -> io::Result<usize>,
) -> io::Result<()> {
    while !data.is_empty() {
        match write(data, offset) {
            Ok(0) => return Err(io::ErrorKind::WriteZero.into()),
            Ok(written) => {
                offset = offset
                    .checked_add(written as u64)
                    .ok_or(io::ErrorKind::InvalidInput)?;
                data = &data[written..];
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, OpenOptions};
    use std::io::{Read, Seek, SeekFrom};

    #[test]
    fn offset_chunks_preserve_contents_and_documented_cursor() {
        let path = std::env::temp_dir().join(format!(
            "rquickshare-offset-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        write_all_at_offset(&file, b"world", 5).unwrap();
        write_all_at_offset(&file, b"hello", 0).unwrap();
        file.seek(SeekFrom::Start(2)).unwrap();
        write_all_at_offset(&file, b"!", 10).unwrap();
        #[cfg(unix)]
        assert_eq!(file.stream_position().unwrap(), 2);
        #[cfg(windows)]
        assert_eq!(file.stream_position().unwrap(), 11);
        write_all_at_offset(&file, b"", 0).unwrap();
        assert_eq!(
            write_all_at_offset(&file, b"xx", u64::MAX)
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidInput
        );
        file.seek(SeekFrom::Start(0)).unwrap();
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();
        assert_eq!(content, b"helloworld!");
        drop(file);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn short_writes_and_interruptions_retry_at_exact_offsets() {
        let mut calls = 0;
        let mut positions = Vec::new();
        write_all_with(b"abcde", 10, |data, position| {
            calls += 1;
            if calls == 1 {
                return Err(io::ErrorKind::Interrupted.into());
            }
            positions.push((position, data.to_vec()));
            Ok(data.len().min(2))
        })
        .unwrap();
        assert_eq!(
            positions,
            vec![
                (10, b"abcde".to_vec()),
                (12, b"cde".to_vec()),
                (14, b"e".to_vec())
            ]
        );
    }

    #[test]
    fn zero_write_and_error_propagate_and_empty_input_does_not_write() {
        assert_eq!(
            write_all_with(b"x", 0, |_, _| Ok(0)).unwrap_err().kind(),
            io::ErrorKind::WriteZero
        );
        assert_eq!(
            write_all_with(b"x", 0, |_, _| Err(io::ErrorKind::PermissionDenied.into()))
                .unwrap_err()
                .kind(),
            io::ErrorKind::PermissionDenied
        );
        write_all_with(b"", 0, |_, _| panic!("empty buffer must not write")).unwrap();
    }
}
