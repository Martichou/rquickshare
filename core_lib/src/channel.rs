use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::hdl::info::TransferMetadata;
use crate::hdl::State;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum ChannelDirection {
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

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct ChannelMessage {
    pub id: String,
    pub direction: ChannelDirection,

    // Only present when channelDirection is frontToLib
    pub action: Option<ChannelAction>,

    // Only present when channelDirection is libToFront
    pub state: Option<State>,
    pub meta: Option<TransferMetadata>,
}
