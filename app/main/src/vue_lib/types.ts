import { State } from '@martichou/core_lib/bindings/State';
import { DeviceType } from '@martichou/core_lib/bindings/DeviceType';
import { Visibility } from '@martichou/core_lib/bindings/Visibility';

export type Theme = 'Light' | 'Dark' | 'System';

export interface ToDelete {
	id: string,
	triggered: number
}

export interface DisplayedItem {
	id: string,
	name: string,
	deviceType: DeviceType,
	endpoint: boolean,

	state?: State,
	pin_code?: string,
	files?: string[],
	text_description?: string,
	text_payload?: string,
	text_type?: string,
	destination?: string,
	total_bytes?: number,
	ack_bytes?: number,
}

export const visibilityToNumber: { [key in Visibility]: number } = {
	'Visible': 0,
	'Invisible': 1,
	'Temporarily': 2,
};

export const numberToVisibility: { [key: number]: Visibility } = {
	0: "Visible",
	1: "Invisible",
	2: "Temporarily",
};

export const autostartKey = "autostart";
export const realcloseKey = "realclose";
export const startminimizedKey = "startminimized";
export const visibilityKey = "visibility";
export const downloadPathKey = "download_path";
export const themeKey = "theme";
export const stateToDisplay: Array<Partial<State>> = ["ReceivedPairedKeyResult", "WaitingForUserConsent", "ReceivingFiles", "Disconnected",
	"Finished", "SentIntroduction", "SendingFiles", "Cancelled", "Rejected"]

export interface Toast {
	id: number;
	type: ToastType;
	message: string;
}

export enum ToastType {
	Success = "SUCCESS",
	Error = "ERROR",
	Info = "INFO",
}