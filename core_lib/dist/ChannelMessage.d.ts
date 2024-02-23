import type { ChannelAction } from "./ChannelAction";
import type { ChannelDirection } from "./ChannelDirection";
import type { State } from "./State";
import type { TransferMetadata } from "./TransferMetadata";
export interface ChannelMessage {
    id: string;
    direction: ChannelDirection;
    action: ChannelAction | null;
    state: State | null;
    meta: TransferMetadata | null;
}
