use std::collections::HashMap;

use info::{InternalFileInfo, TransferMetadata};
use p256::{PublicKey, SecretKey};

use crate::securegcm::ukey2_client_init::CipherCommitment;
use crate::sharing_nearby::wifi_credentials_metadata::SecurityType;
use crate::utils::RemoteDeviceInfo;

#[cfg(feature = "experimental")]
mod ble;
#[cfg(feature = "experimental")]
pub use ble::*;
#[cfg(all(feature = "experimental", target_os = "linux"))]
mod blea;
#[cfg(all(feature = "experimental", target_os = "linux"))]
pub use blea::*;
mod inbound;
pub use inbound::*;
pub mod info;
mod mdns_discovery;
pub use mdns_discovery::*;
mod mdns;
pub use mdns::*;
mod outbound;
pub use outbound::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq)]
pub enum TransferState {
    #[default]
    Initial,
    ReceivedConnectionRequest,
    SentUkeyServerInit,
    SentUkeyClientInit,
    SentUkeyClientFinish,
    SentPairedKeyEncryption,
    ReceivedUkeyClientFinish,
    SentConnectionResponse,
    SentPairedKeyResult,
    SentIntroduction,
    ReceivedPairedKeyResult,
    WaitingForUserConsent,
    ReceivingFiles,
    SendingFiles,
    Disconnected,
    Rejected,
    Cancelled,
    Finished,
}

#[derive(Debug, Default)]
pub struct InnerState {
    pub id: String,
    pub server_seq: i32,
    pub client_seq: i32,
    pub encryption_done: bool,

    // Subject to be used-facing for progress, ...
    pub state: TransferState,
    pub remote_device_info: Option<RemoteDeviceInfo>,
    pub pin_code: Option<String>,
    pub transfer_metadata: Option<TransferMetadata>,
    pub transferred_files: HashMap<i64, InternalFileInfo>,

    // Everything needed for encryption/decryption/verif
    pub cipher_commitment: Option<CipherCommitment>,
    pub private_key: Option<SecretKey>,
    pub public_key: Option<PublicKey>,
    pub server_init_data: Option<Vec<u8>>,
    pub client_init_msg_data: Option<Vec<u8>>,
    pub ukey_client_finish_msg_data: Option<Vec<u8>>,
    pub decrypt_key: Option<Vec<u8>>,
    pub recv_hmac_key: Option<Vec<u8>>,
    pub encrypt_key: Option<Vec<u8>>,
    pub send_hmac_key: Option<Vec<u8>>,

    // Used to handle/track ingress transfer
    pub text_payload: Option<TextPayloadInfo>,
    // pub text_payload_id: i64,
    // pub text_is_url: bool,
    // pub wifi_ssid: Option<String>,
    pub payload_buffers: HashMap<i64, Vec<u8>>,
}

#[derive(Debug, Clone)]
pub enum TextPayloadInfo {
    Url(i64),
    Text(i64),
    Wifi((i64, String, SecurityType)), // id, ssid, security type
}

#[derive(Debug, Clone)]
pub enum TextPayloadType {
    Url,
    Text,
    Wifi,
}

impl TextPayloadInfo {
    fn get_i64_value(&self) -> i64 {
        match self {
            TextPayloadInfo::Url(value)
            | TextPayloadInfo::Text(value)
            | TextPayloadInfo::Wifi((value, _, _)) => value.to_owned(),
        }
    }
}
