use std::fs::File;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{sharing_nearby::FileMetadata, utils::RemoteDeviceInfo};

#[derive(Debug)]
pub struct InternalFileInfo {
    pub meta: FileMetadata,
    pub payload_id: i64,
    pub destination_url: PathBuf,
    pub bytes_transferred: i64,
    pub file: Option<File>,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct TransferMetadata {
    pub id: String,
    pub destination: Option<String>,
    pub source: Option<RemoteDeviceInfo>,
    pub files: Option<Vec<String>>,
    pub pin_code: Option<String>,
    pub text_description: Option<String>,
}
