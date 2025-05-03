use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-support")]
use ts_rs::TS;

use crate::hdl::info::TransferMetadata;
use crate::hdl::State;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "ts-support", derive(TS))]
#[cfg_attr(feature = "ts-support", ts(export))]
pub enum ChannelDirection {
    #[default]
    FrontToLib,
    LibToFront,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "ts-support", derive(TS))]
#[cfg_attr(feature = "ts-support", ts(export))]
pub enum ChannelAction {
    AcceptTransfer,
    RejectTransfer,
    CancelTransfer,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "ts-support", derive(TS))]
#[cfg_attr(feature = "ts-support", ts(export))]
pub enum TransferType {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "ts-support", derive(TS))]
#[cfg_attr(feature = "ts-support", ts(export))]
pub struct ChannelMessage {
    pub id: String,
    pub direction: ChannelDirection,

    // Only present when channelDirection is frontToLib
    pub action: Option<ChannelAction>,

    // Only present when channelDirection is libToFront
    pub rtype: Option<TransferType>,
    pub state: Option<State>,
    pub meta: Option<TransferMetadata>,
}
