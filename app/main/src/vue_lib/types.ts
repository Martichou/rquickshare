import { State } from '@martichou/core_lib/bindings/State';
import { DeviceType } from '@martichou/core_lib/bindings/DeviceType';
import { Visibility } from '@martichou/core_lib/bindings/Visibility';

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
export const autoAcceptTimeoutKey = "auto_accept_timeout";
export const trustedDevicesKey = "trusted_devices";
export const blockedDevicesKey = "blocked_devices";

export type AutoAcceptTimeout = 1 | 5 | 10 | 60;

export const autoAcceptTimeoutOptions: { value: AutoAcceptTimeout; label: string }[] = [
	{ value: 1, label: '1 minute' },
	{ value: 5, label: '5 minutes' },
	{ value: 10, label: '10 minutes' },
	{ value: 60, label: '1 hour' },
];

export interface TrustedDevice {
	name: string;
	deviceType: string;
	lastTransfer: number;
}

export interface BlockedDevice {
	name: string;
	deviceType: string;
	blockedAt: number;
}

export interface PendingTrustRequest {
	id: string;
	name: string;
	deviceType: string;
}
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