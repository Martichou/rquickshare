import type { DeviceType } from "./DeviceType";
export interface EndpointInfo {
    id: string;
    name: string | null;
    ip: string | null;
    port: string | null;
    rtype: DeviceType | null;
    present: boolean | null;
}
