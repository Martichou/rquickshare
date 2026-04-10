use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};

pub async fn write_frame(writer: &mut (impl AsyncWrite + Unpin), data: &[u8]) -> Result<(), anyhow::Error> {
        let length = data.len();

        // Prepare length prefix in big-endian format
        let length_bytes = [
            (length >> 24) as u8,
            (length >> 16) as u8,
            (length >> 8) as u8,
            length as u8,
        ];

        let mut prefixed_length = Vec::with_capacity(length + 4);
        prefixed_length.extend_from_slice(&length_bytes);
        prefixed_length.extend_from_slice(&data);

        writer.write_all(&prefixed_length).await?;
        writer.flush().await?;

        Ok(())
}

pub async fn read_frame_header(
    reader: &mut (impl AsyncRead + Unpin),
) -> Result<usize, anyhow::Error> {
    let mut length_buf = [0u8; 4];
    reader.read_exact(&mut length_buf).await?;
    Ok(u32::from_be_bytes(length_buf) as usize)
}

pub async fn read_frame_payload(
    reader: &mut (impl AsyncRead + Unpin),
    length: usize,
) -> Result<Vec<u8>, anyhow::Error> {
    let mut payload = vec![0u8; length];
    reader.read_exact(&mut payload).await?;
    Ok(payload)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
    async fn test_frame_handling () {
        let (mut sender, mut receiver) = tokio::io::duplex(1024);
        let original_data = vec![10, 20, 30, 40, 50];
        write_frame(&mut sender, &original_data).await.unwrap();
        let received_length = read_frame_header(&mut receiver).await.unwrap();
        
        assert_eq!(received_length,original_data.len(), "Recieved length is not the same as the original length!");

        let received_data = read_frame_payload(&mut receiver, received_length).await.unwrap();
        assert_eq!(received_data,original_data, "Recieved data is not the same as the original data!");
    }
}