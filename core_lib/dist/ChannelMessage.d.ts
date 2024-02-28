import type { ChannelAction } from "./ChannelAction";
import type { ChannelDirection } from "./ChannelDirection";
import type { State } from "./State";
import type { TransferMetadata } from "./TransferMetadata";
import type { TransferType } from "./TransferType";
export interface ChannelMessage {
    id: string;
    direction: ChannelDirection;
    action: ChannelAction | null;
    rtype: TransferType | null;
    state: State | null;
    meta: TransferMetadata | null;
}
