use crate::hdl::info::TransferMetadata;
use crate::hdl::State;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ChannelDirection {
    #[default]
    FrontToLib,
    LibToFront,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelAction {
    AcceptTransfer,
    RejectTransfer,
    CancelTransfer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransferType {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Default)]
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
