import type { RemoteDeviceInfo } from "./RemoteDeviceInfo";
export interface TransferMetadata {
    id: string;
    destination: string | null;
    source: RemoteDeviceInfo | null;
    files: Array<string> | null;
    pin_code: string | null;
    text_description: string | null;
}
