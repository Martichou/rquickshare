use std::sync::Mutex;
#[cfg(any(target_os = "macos", test))]
use tauri::Url;

#[cfg(target_os = "macos")]
use tauri::{AppHandle, Emitter, Manager};

#[cfg(target_os = "macos")]
use crate::open_main_window;

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ShareRequest {
    pub files: Vec<String>,
}

/// Managed on the builder rather than in AppState: macOS can deliver files at
/// launch before setup has run.
#[derive(Default)]
pub struct PendingShare(Mutex<Option<ShareRequest>>);

#[tauri::command]
pub fn take_pending_share(state: tauri::State<'_, PendingShare>) -> Option<ShareRequest> {
    state.0.lock().ok().and_then(|mut pending| pending.take())
}

/// Files macOS hands to the app, from the Share extension, "Open With" or a
/// drop on the Dock icon. They wait in `PendingShare` until the front end
/// takes them, so files that arrive before the window has loaded aren't lost.
#[cfg(target_os = "macos")]
pub fn handle_opened(app: &AppHandle, urls: &[Url]) {
    let files = files_from_urls(urls);
    if files.is_empty() {
        return;
    }
    info!("share: {} file(s) opened with the app", files.len());

    let state = app.state::<PendingShare>();
    let request = {
        let Ok(mut pending) = state.0.lock() else {
            warn!(
                "share: pending share lock poisoned, dropped {} file(s)",
                files.len()
            );
            return;
        };
        let request = pending.get_or_insert_with(ShareRequest::default);
        for file in files {
            if !request.files.contains(&file) {
                request.files.push(file);
            }
        }
        request.clone()
    };

    if let Err(e) = app.emit("share_received", &request) {
        warn!("share: couldn't tell the front end: {e}");
    }
    open_main_window(app);
}

/// The existing files among `urls`. Anything that isn't a local file is skipped.
#[cfg(any(target_os = "macos", test))]
fn files_from_urls(urls: &[Url]) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for url in urls {
        let Some(path) = url.to_file_path().ok().filter(|path| path.is_file()) else {
            warn!("share: not a local file, skipped: {url}");
            continue;
        };
        let path = path.to_string_lossy().into_owned();
        if !files.contains(&path) {
            files.push(path);
        }
    }
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_existing_files_once() {
        let dir = std::env::temp_dir().join(format!("rqs-share-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("a b.txt");
        std::fs::write(&file, b"x").unwrap();
        let url = Url::from_file_path(&file).unwrap();

        let files = files_from_urls(&[url.clone(), url]);
        assert_eq!(files, vec![file.to_string_lossy()]);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn skips_missing_files_folders_and_other_schemes() {
        let urls = [
            Url::parse("file:///definitely/not/here.txt").unwrap(),
            Url::from_directory_path(std::env::temp_dir()).unwrap(),
            Url::parse("https://example.com/x.txt").unwrap(),
        ];
        assert!(files_from_urls(&urls).is_empty());
    }
}
