import { Visibility } from '@martichou/core_lib/bindings/Visibility';
import { TauriVM } from './h/ParamsHelper';
import { autostartKey, DisplayedItem, downloadPathKey, numberToVisibility, realcloseKey, stateToDisplay, visibilityKey, visibilityToNumber } from './types';
import { SendInfo } from '@martichou/core_lib/bindings/SendInfo';
import { ChannelMessage } from '@martichou/core_lib/bindings/ChannelMessage';
import { ChannelAction } from '@martichou/core_lib';

export * from './types';
export * from './stores/useToastStore';

export { default as ToastNotification } from './components/ToastNotification.vue';

export function opt<T>(v?: T) {
	return v ?? null;
}

export function _displayedItems(vm: TauriVM): Array<DisplayedItem> {
	const ndisplayed = new Array<DisplayedItem>();

	vm.endpointsInfo.forEach((el) => {
		const idx = ndisplayed.findIndex((nel) => el.id == nel.id);
		if (idx !== -1) return;

		ndisplayed.push({
			id: el.id,
			name: el.name ?? 'Unknown',
			deviceType: el.rtype ?? 'Unknown',
			endpoint: true,
		})
	});

	vm.requests.filter((el) => stateToDisplay.includes(el.state ?? 'Initial')).forEach((el) => {
		const idx = ndisplayed.findIndex((nel) => el.id == nel.id);
		const elem: DisplayedItem = {
			id: el.id,
			name: el.meta?.source?.name ?? 'Unknown',
			deviceType: el.meta?.source?.device_type ?? 'Unknown',
			endpoint: false,

			state: el.state ?? undefined,
			pin_code: el.meta?.pin_code ?? undefined,
			destination: el.meta?.destination ?? el.meta?.text_payload ?? undefined,
			files: el.meta?.files ?? undefined,
			text_description: el.meta?.text_description ?? undefined,
			ack_bytes: (el.meta?.ack_bytes as number | undefined) ?? undefined,
			total_bytes: (el.meta?.total_bytes as number | undefined) ?? undefined,
		};

		if (idx !== -1) {
			ndisplayed.splice(idx, 1, elem);
		} else {
			ndisplayed.push(elem)
		}
	});

	return ndisplayed;
}

export async function setAutoStart(vm: TauriVM, autostart: boolean) {
	if (autostart) {
		await vm.enable();
	} else {
		await vm.disable();
	}

	await vm.store.set(autostartKey, autostart);
	await vm.store.save();
	vm.autostart = autostart;
}

export async function applyAutoStart(vm: TauriVM) {
	vm.autostart = await vm.store.get(autostartKey) ?? false;

	if (vm.autostart) {
		await vm.enable();
	} else {
		await vm.disable();
	}
}

export async function setRealClose(vm: TauriVM, realclose: boolean) {
	await vm.store.set(realcloseKey, realclose);
	await vm.store.save();
	vm.realclose = realclose;
}

export async function getRealclose(vm: TauriVM) {
	vm.realclose = await vm.store.get(realcloseKey) ?? false;
}

export async function setVisibility(vm: TauriVM, visibility: Visibility) {
	await vm.invoke('change_visibility', { message: visibility });
	await vm.store.set(visibilityKey, visibilityToNumber[visibility]);
	await vm.store.save();
	vm.visibility = visibility;
}

export async function getVisibility(vm: TauriVM) {
	vm.visibility = numberToVisibility[(await vm.store.get(visibilityKey) ?? 0) as number];
}

export async function invertVisibility(vm: TauriVM) {
	if (vm.visibility === 'Temporarily') {
		return;
	}

	if (vm.visibility === 'Visible') {
		return await vm.setVisibility(vm, 'Invisible');
	}

	return await vm.setVisibility(vm, 'Visible');
}

export async function clearSending(vm: TauriVM, ) {
	await vm.invoke('stop_discovery');
	vm.outboundPayload = undefined;
	vm.discoveryRunning = false;
	vm.endpointsInfo = [];
}

export function removeRequest(vm: TauriVM, id: string) {
	const idx = vm.requests.findIndex((el) => el.id === id);

	if (idx !== -1) {
		vm.requests.splice(idx, 1);
	}
}

export async function sendInfo(vm: TauriVM, eid: string) {
	if (vm.outboundPayload === undefined) return;

	const ei = vm.endpointsInfo.find((el) => el.id === eid);
	if (!ei || !ei.ip || !ei.port) return;

	const msg: SendInfo = {
		id: ei.id,
		name: ei.name ?? 'Unknown',
		addr: ei.ip + ":" + ei.port,
		ob: vm.outboundPayload,
	};

	await vm.invoke('send_payload', { message: msg });
}

export async function sendCmd(vm: TauriVM, id: string, action: ChannelAction) {
	const cm: ChannelMessage = {
		id: id,
		direction: 'FrontToLib',
		action: action,
		meta: null,
		state: null,
		rtype: null,
	};
	console.log("js2rs:", cm);

	await vm.invoke('send_to_rs', { message: cm });
}

export function blured() {
	(document.activeElement as any).blur();
}

export function getProgress(item: DisplayedItem): string {
	const value = item.ack_bytes! / item.total_bytes! * 100;
	return `--progress: ${value}`;
}

export async function setDownloadPath(vm: TauriVM, dest: string) {
	await vm.invoke('change_download_path', { message: dest });
	await vm.store.set(downloadPathKey, dest);
	await vm.store.save();
	vm.downloadPath = dest;
}

export async function getDownloadPath(vm: TauriVM) {
	vm.downloadPath = await vm.store.get(downloadPathKey) ?? undefined;
}

export async function getLatestVersion(vm: TauriVM, gt: (a: string, b: string) => boolean) {
	try {
		const response = await fetch('https://api.github.com/repos/martichou/rquickshare/releases/latest');
		if (!response.ok) {
			throw new Error(`Error: ${response.status} ${response.statusText}`);
		}
		const data = await response.json();
		const new_version = data.tag_name.substring(1);
		console.log(`Latest version: ${vm.version} vs ${new_version}`);

		if (vm.version && gt(new_version, vm.version)) {
			vm.new_version = new_version;
		}
	} catch (err) {
		console.error(err);
	}
}