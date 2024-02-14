use std::collections::HashMap;

use p256::{PublicKey, SecretKey};

use self::info::{InternalFileInfo, TransferMetadata};
use crate::securegcm::ukey2_client_init::CipherCommitment;
use crate::utils::RemoteDeviceInfo;

mod inbound;
pub use inbound::*;
mod info;
mod mdns;
pub use mdns::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum State {
    Initial,
    ReceivedConnectionRequest,
    SentUkeyServerInit,
    ReceivedUkeyClientFinish,
    SentConnectionResponse,
    SentPairedKeyResult,
    ReceivedPairedKeyResult,
    WaitingForUserConsent,
    ReceivingFiles,
    Disconnected,
}

#[derive(Debug)]
pub struct InnerState {
    pub id: String,
    pub server_seq: i32,
    pub client_seq: i32,
    pub state: State,
    pub encryption_done: bool,
    pub remote_device_info: Option<RemoteDeviceInfo>,
    pub cipher_commitment: Option<CipherCommitment>,
    pub private_key: Option<SecretKey>,
    pub public_key: Option<PublicKey>,
    pub server_init_data: Option<Vec<u8>>,
    pub client_init_msg_data: Option<Vec<u8>>,
    pub decrypt_key: Option<Vec<u8>>,
    pub recv_hmac_key: Option<Vec<u8>>,
    pub encrypt_key: Option<Vec<u8>>,
    pub send_hmac_key: Option<Vec<u8>>,
    pub pin_code: Option<String>,

    pub text_payload_id: i64,
    pub payload_buffers: HashMap<i64, Vec<u8>>,
    pub transfer_metadata: Option<TransferMetadata>,
    pub transferred_files: HashMap<i64, InternalFileInfo>,
}
