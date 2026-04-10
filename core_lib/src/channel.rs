use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::hdl::info::TransferMetadata;
use crate::hdl::State;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum ChannelDirection {
    #[default]
    FrontToLib,
    LibToFront,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum ChannelAction {
    AcceptTransfer,
    RejectTransfer,
    CancelTransfer,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum TransferType {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type", content = "data")]
pub enum ChannelError {
    // Add other specific variants here as needed
    Generic { message: String, debug: String },
}

pub trait ToChannelError {
    fn to_channel_error(&self) -> ChannelError;
}

impl<E: std::fmt::Display + std::fmt::Debug> ToChannelError for E {
    fn to_channel_error(&self) -> ChannelError {
        ChannelError::Generic {
            message: self.to_string(),
            debug: format!("{:?}", self),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct ChannelMessage {
    pub id: String,
    pub direction: ChannelDirection,

    // Only present when channelDirection is frontToLib
    pub action: Option<ChannelAction>,

    // Only present when channelDirection is libToFront
    pub rtype: Option<TransferType>,
    pub state: Option<State>,
    pub meta: Option<TransferMetadata>,
    pub error: Option<ChannelError>,
}

impl ChannelMessage {
    pub fn set_error<E: ToChannelError>(&mut self, err: E) {
        self.error = Some(err.to_channel_error());
    }
    pub fn with_error<E: ToChannelError>(mut self, err: E) -> Self {
        self.set_error(err);
        self
    }
}
