import { Store } from '@tauri-apps/plugin-store';
import { UnlistenFn } from '@tauri-apps/api/event';

import { ToDelete } from '../types';

import { EndpointInfo } from '@martichou/core_lib/bindings/EndpointInfo';
import { Visibility } from '@martichou/core_lib/bindings/Visibility';
import { OutboundPayload } from '@martichou/core_lib/bindings/OutboundPayload';
import { ChannelMessage } from '@martichou/core_lib/bindings/ChannelMessage';
import { Theme } from '../types';

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
    startminimized: boolean;
    visibility: Visibility;
    theme: Theme;
    downloadPath: string | undefined;
    hostname: string | undefined;
    settingsOpen: boolean;
    new_version: string | null;
    enable: () => Promise<void>;
    disable: () => Promise<void>;
    invoke: (cmd: string, args?: InvokeArgs) => Promise<unknown>
    setVisibility: (vm: TauriVM, visibility: Visibility) => Promise<void>;

    displayedIsEmpty: boolean;
    displayedItems: DisplayedItem[];

    // Remapped function for compatibility with Tauri v1 and v2
    dialogOpen: (options?: {
        title: string,
        directory: boolean,
        multiple: boolean,
    }) => Promise<unknown>;
}