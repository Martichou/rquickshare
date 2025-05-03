use std::fs::File;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-support")]
use ts_rs::TS;

use super::TextPayloadType;
use crate::utils::RemoteDeviceInfo;

#[derive(Debug)]
pub struct InternalFileInfo {
    pub payload_id: i64,
    pub file_url: PathBuf,
    pub bytes_transferred: i64,
    pub total_size: i64,
    pub file: Option<File>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "ts-support", derive(TS))]
#[cfg_attr(feature = "ts-support", ts(export))]
pub struct TransferMetadata {
    pub id: String,
    pub source: Option<RemoteDeviceInfo>,
    pub pin_code: Option<String>,

    pub destination: Option<String>,
    pub files: Option<Vec<String>>,

    pub text_type: Option<TextPayloadType>,
    pub text_description: Option<String>,
    pub text_payload: Option<String>,

    pub total_bytes: u64,
    pub ack_bytes: u64,
}
