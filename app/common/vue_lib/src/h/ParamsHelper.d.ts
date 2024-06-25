import { Store } from '@tauri-apps/plugin-store';
import { UnlistenFn } from '@tauri-apps/api/event';

import { ToDelete } from '../types';

import { EndpointInfo } from '@martichou/core_lib/bindings/EndpointInfo';
import { Visibility } from '@martichou/core_lib/bindings/Visibility';
import { OutboundPayload } from '@martichou/core_lib/bindings/OutboundPayload';
import { ChannelMessage } from '@martichou/core_lib/bindings/ChannelMessage';

export interface TauriVM {
	store: Store;
    isAppInForeground: boolean;
    discoveryRunning: boolean;
    isDragHovering: boolean;
    requests: ChannelMessage[];
    endpointsInfo: EndpointInfo[];
    toDelete: ToDelete[];
    outboundPayload: OutboundPayload | undefined;
    unlisten: Array<UnlistenFn>;
    version: string | null;
    autostart: boolean;
    realclose: boolean;
    visibility: Visibility;
    downloadPath: string | undefined;
    hostname: string | undefined;
    settingsOpen: boolean;
    enable: () => Promise<void>;
    disable: () => Promise<void>;
    invoke: (cmd: string, args?: InvokeArgs) => Promise<unknown>
    setVisibility: (vm: TauriVM, visibility: Visibility) => Promise<void>;
}