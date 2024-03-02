<template>
	<div class="flex flex-col bg-green-50 w-full h-full max-w-full max-h-full">
		<div class="flex flex-row justify-between items-center px-6 py-4">
			<!-- Header, Pc name left and options right -->
			<div>
				<h4 class="text-md">
					Device name
				</h4>
				<h2 class="text-2xl font-medium">
					{{ hostname }}
				</h2>
			</div>
			<div class="flex justify-center items-center gap-4">
				<p class="text-sm">
					v{{ version }}
				</p>
				<details class="dropdown dropdown-end">
					<summary class="hover:bg-gray-200 cursor-pointer p-2 rounded-lg active:scale-105 transition duration-150 ease-in-out">
						<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24">
							<!-- eslint-disable-next-line -->
							<path d="M480-160q-33 0-56.5-23.5T400-240q0-33 23.5-56.5T480-320q33 0 56.5 23.5T560-240q0 33-23.5 56.5T480-160Zm0-240q-33 0-56.5-23.5T400-480q0-33 23.5-56.5T480-560q33 0 56.5 23.5T560-480q0 33-23.5 56.5T480-400Zm0-240q-33 0-56.5-23.5T400-720q0-33 23.5-56.5T480-800q33 0 56.5 23.5T560-720q0 33-23.5 56.5T480-640Z" />
						</svg>
					</summary>
					<ul class="mt-2 p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box w-56">
						<li>
							<span class="active:!bg-green-100 active:!text-black" v-if="autostart" @click="setAutostart(false)">
								Disable start on boot
							</span>
							<span class="active:!bg-green-100 active:!text-black" v-else @click="setAutostart(true)">
								Enable start on boot
							</span>
						</li>
					</ul>
				</details>
			</div>
		</div>
		<div class="flex-1 flex flex-row">
			<!-- Content -->
			<!-- When uploading: left info about current file being uploaded -->
			<!-- 				 right, list of nearby devices -->
			<!-- Default: left settings about visibility -->
			<!-- 		  right, ready to receive with hint for drag & drop, then request (to accept or not) -->
			<div class="w-72 p-6" v-if="outboundPayload === undefined">
				<p class="mt-4 mb-2">
					Currently
				</p>
				<h4 class="font-medium">
					Receiving from everyone
				</h4>
				<p class="text-sm mt-1">
					Everyone can share with you (you still need to approve each transfer).
				</p>
			</div>
			<div class="w-72 p-6 flex flex-col justify-between" v-else>
				<div>
					<p class="mt-4 mb-2">
						Sharing {{ outboundPayload.Files.length }} file{{ outboundPayload.Files.length > 1 ? 's' : '' }}
					</p>
					<div class="w-32 h-32 rounded-2xl bg-white mb-2 flex justify-center items-center">
						<svg
							xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
							class="fill-gray-700 w-8 h-8">
							<!-- eslint-disable-next-line -->
							<path d="M240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v480q0 33-23.5 56.5T720-80H240Zm280-520v-200H240v640h480v-440H520ZM240-800v200-200 640-640Z" />
						</svg>
					</div>
					<p v-for="f in outboundPayload.Files" :key="f" class="overflow-hidden whitespace-nowrap text-ellipsis">
						{{ f.split('/').pop() }}
					</p>

					<p class="text-xs mt-3">
						Make sure both devices are unlocked, close together, and have bluetooth turned on. Device you're sharing with need
						Quick Share turned on and visible to you.
					</p>
				</div>

				<p
					@click="clearSending()"
					class="outline outline-1 outline-gray-700 cursor-pointer p-1 px-3 rounded-full
					font-medium active:scale-105 transition duration-150 ease-in-out w-fit">
					Cancel
				</p>
			</div>
			<div
				class="flex-1 flex flex-col h-full rounded-tl-[3rem] bg-white p-12"
				:class="{'items-center': displayedIsEmpty}">
				<h3 class="mb-4 font-medium text-xl">
					<span v-if="displayedIsEmpty">Ready to receive{{ outboundPayload != undefined ? ' / send' : '' }}</span>
					<span v-else>Nearby devices</span>
				</h3>

				<div v-if="displayedIsEmpty && endpointsInfo.length === 0" class="my-auto status-indicator status-indicator--success status-indicator--xl">
					<div class="circle circle--animated circle-main" />
					<div class="circle circle--animated circle-secondary" />
					<div class="circle circle--animated circle-tertiary" />
				</div>

				<div
					v-if="displayedIsEmpty && outboundPayload === undefined" class="w-full border-dashed border-2 border-gray-300 rounded-2xl p-6 flex flex-col
						justify-center items-center transition duration-150 ease-in-out"
					:class="{'border-green-200': isDragHovering, 'bg-green-100': isDragHovering, 'scale-105': isDragHovering}">
					<svg
						xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
						class="fill-gray-700 w-8 h-8">
						<!-- eslint-disable-next-line -->
						<path d="M440-320v-326L336-542l-56-58 200-200 200 200-56 58-104-104v326h-80ZM240-160q-33 0-56.5-23.5T160-240v-120h80v120h480v-120h80v120q0 33-23.5 56.5T720-160H240Z" />
					</svg>
					<h4 class="mt-2 font-medium">
						Drop files to send
					</h4>
				</div>

				<div
					v-for="item in displayedItems" :key="item.id"
					class="w-full bg-green-200 bg-opacity-65 rounded-3xl flex flex-row gap-6 p-4 mb-4"
					:class="{'cursor-pointer': item.endpoint}" @click="item.endpoint && sendInfo(item.id)">
					<div>
						<div class="h-16 w-16 rounded-full bg-green-50" :class="{'!bg-green-400': item.state === 'Finished'}">
							<svg
								v-if="item.state === 'Finished'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4 fill-white">
								<!-- eslint-disable-next-line -->
								<path d="M268-240 42-466l57-56 170 170 56 56-57 56Zm226 0L268-466l56-57 170 170 368-368 56 57-424 424Zm0-226-57-56 198-198 57 56-198 198Z" />
							</svg>
							<svg
								v-else-if="item.deviceType === 'Laptop'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4 fill-gray-900">
								<!-- eslint-disable-next-line -->
								<path d="M0-160v-80h160v-40q-33 0-56.5-23.5T80-360v-400q0-33 23.5-56.5T160-840h640q33 0 56.5 23.5T880-760v400q0 33-23.5 56.5T800-280v40h160v80H0Zm160-200h640v-400H160v400Zm0 0v-400 400Z" />
							</svg>
							<svg
								v-else-if="item.deviceType === 'Phone'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4 fill-gray-900">
								<!-- eslint-disable-next-line -->
								<path d="M280-40q-33 0-56.5-23.5T200-120v-720q0-33 23.5-56.5T280-920h400q33 0 56.5 23.5T760-840v720q0 33-23.5 56.5T680-40H280Zm0-120v40h400v-40H280Zm0-80h400v-480H280v480Zm0-560h400v-40H280v40Zm0 0v-40 40Zm0 640v40-40Z" />
							</svg>
							<svg
								v-else-if="item.deviceType === 'Tablet'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4 fill-gray-900">
								<!-- eslint-disable-next-line -->
								<path d="M120-160q-33 0-56.5-23.5T40-240v-480q0-33 23.5-56.5T120-800h720q33 0 56.5 23.5T920-720v480q0 33-23.5 56.5T840-160H120Zm40-560h-40v480h40v-480Zm80 480h480v-480H240v480Zm560-480v480h40v-480h-40Zm0 0h40-40Zm-640 0h-40 40Z" />
							</svg>
							<svg
								v-else xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4 fill-gray-900">
								<!-- eslint-disable-next-line -->
								<path d="M280-160H160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h640v80H160v480h120v80Zm160-100q25 0 42.5-17.5T500-320q0-25-17.5-42.5T440-380q-25 0-42.5 17.5T380-320q0 25 17.5 42.5T440-260Zm-80 100v-71q-19-17-29.5-40T320-320q0-26 10.5-49t29.5-40v-71h160v71q19 17 29.5 40t10.5 49q0 26-10.5 49T520-231v71H360Zm480 0H640q-17 0-28.5-11.5T600-200v-360q0-17 11.5-28.5T640-600h200q17 0 28.5 11.5T880-560v360q0 17-11.5 28.5T840-160Zm-160-80h120v-280H680v280Zm0 0h120-120Z" />
							</svg>
						</div>

						<p
							v-if="(item.state === 'WaitingForUserConsent' || item.state === 'SentIntroduction') && item.pin_code"
							class="text-center inline-flex gap-1 mt-4 text-sm items-center">
							<svg
								xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
								class="fill-gray-900">
								<!-- eslint-disable-next-line -->
								<path d="M420-360h120l-23-129q20-10 31.5-29t11.5-42q0-33-23.5-56.5T480-640q-33 0-56.5 23.5T400-560q0 23 11.5 42t31.5 29l-23 129Zm60 280q-139-35-229.5-159.5T160-516v-244l320-120 320 120v244q0 152-90.5 276.5T480-80Zm0-84q104-33 172-132t68-220v-189l-240-90-240 90v189q0 121 68 220t172 132Zm0-316Z" />
							</svg>
							{{ item.pin_code }}
						</p>
					</div>
					<div class="flex-1 flex flex-col text-sm" :class="{'justify-center': item.state === undefined}">
						<h4 class="text-base font-medium">
							{{ item.name }}
						</h4>

						<div v-if="item.state === 'WaitingForUserConsent'" class="flex-1 flex flex-col justify-between">
							<p class="mt-4">
								Wants to share {{ item.files?.join(', ') ?? item.text_description ?? 'some file(s).' }}
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="sendCmd(item.id, 'AcceptTransfer')"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-full
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Accept
								</p>
								<p
									@click="sendCmd(item.id, 'RejectTransfer')"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-full
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Decline
								</p>
							</div>
						</div>

						<div v-else-if="['SentIntroduction', 'SendingFiles', 'ReceivingFiles'].includes(item.state ?? 'Initial')">
							<p class="mt-2" v-if="['SentIntroduction', 'SendingFiles'].includes(item.state ?? 'Initial')">
								Sending...
							</p>
							<p class="mt-2" v-else>
								Receiving...
							</p>
							<p v-for="f in item.files ?? []" :key="f" class="overflow-hidden whitespace-nowrap text-ellipsis">
								{{ f }}
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="sendCmd(item.id, 'CancelTransfer')"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-full
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Cancel
								</p>
							</div>
						</div>

						<div v-else-if="item.state === 'Finished'">
							<p class="mt-2">
								Received
							</p>
							<p v-if="item.destination">
								Saved to {{ item.destination }}
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									v-if="item.destination"
									@click="invoke('open', { message: item.destination })"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-full
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Open
								</p>
								<p
									@click="removeRequest(item.id)"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-full
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Clear
								</p>
							</div>
						</div>

						<div v-else-if="item.state === 'Cancelled'">
							<p class="mt-2">
								Transfer cancelled
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="removeRequest(item.id)"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-full
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Clear
								</p>
							</div>
						</div>

						<div v-else-if="item.state === 'Rejected'">
							<p class="mt-2">
								Transfer rejected
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="removeRequest(item.id)"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-full
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Clear
								</p>
							</div>
						</div>

						<div v-else-if="item.state === 'Disconnected'">
							<p class="mt-2">
								Unexpected disconnection
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="removeRequest(item.id)"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-full
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Clear
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script lang="ts">
import { ref, nextTick } from 'vue'
import { UnlistenFn, listen } from '@tauri-apps/api/event'
import { invoke } from "@tauri-apps/api/core"
import { getCurrent } from '@tauri-apps/api/webview';
import { getVersion } from '@tauri-apps/api/app';
import { Store } from "@tauri-apps/plugin-store";
import { disable, enable } from "@tauri-apps/plugin-autostart";
import { isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';

import { opt } from '../utils';
import { ChannelMessage } from '../../../core_lib/bindings/ChannelMessage';
import { ChannelAction } from '../../../core_lib/bindings/ChannelAction';
import { EndpointInfo } from '../../../core_lib/dist/EndpointInfo';
import { OutboundPayload } from '../../../core_lib/bindings/OutboundPayload';
import { SendInfo } from '../../../core_lib/bindings/SendInfo';
import { State } from '../../../core_lib/bindings/State';
import { DeviceType } from '../../../core_lib/bindings/DeviceType';

interface ToDelete {
	id: string,
	triggered: number
}

interface DisplayedItem {
	id: string,
	name: string,
	deviceType: DeviceType,
	endpoint: boolean,

	state?: State,
	pin_code?: string,
	files?: string[],
	text_description?: string,
	destination?: string,
}

const autostartKey = "autostart";
const stateToDisplay: Array<Partial<State>> = ["ReceivedPairedKeyResult", "WaitingForUserConsent", "ReceivingFiles", "Disconnected", "Finished", "SentIntroduction", "SendingFiles", "Cancelled", "Rejected"]

export default {
	name: "HomePage",

	setup() {
		const store = new Store(".settings.json");

		return {stateToDisplay, invoke, getVersion, store};
	},

	data() {
		return {
			isAppInForeground: false,
			discoveryRunning: ref(false),
			isDragHovering: ref(false),

			requests: ref<ChannelMessage[]>([]),
			endpointsInfo: ref<EndpointInfo[]>([]),
			toDelete: ref<ToDelete[]>([]),
			outboundPayload: ref<OutboundPayload | undefined>(),

			cleanupInterval: opt<NodeJS.Timeout>(),
			unlisten: Array<UnlistenFn>(),

			version: opt<string>(),

			autostart: ref<boolean>(true),

			hostname: ref<string>()
		};
	},

	mounted: function () {
		nextTick(async () => {
			this.hostname = await invoke('get_hostname');
			this.version = await getVersion();

			if (!await this.store.has(autostartKey)) {
				await this.setAutostart(true);
			} else {
				await this.applyAutostart();
			}

			// Check permission for notification
			let permissionGranted = await isPermissionGranted();
			if (!permissionGranted) {
				const permission = await requestPermission();
				permissionGranted = permission === 'granted';
			}

			this.cleanupInterval = setInterval(() => {
				this.toDelete.forEach((itemToDelete) => {
					const now = new Date();
					const timeDifference = now.getTime() - itemToDelete.triggered;

					// Check if at least 30 seconds have passed (30000 milliseconds)
					if (timeDifference >= 30000) this.removeRequest(itemToDelete.id);
				});

				// Clear only elements that have been processed (more than 30s old)
				this.toDelete = this.toDelete.filter((item) => {
					const now = new Date();
					return now.getTime() - item.triggered < 30000;
				});
			}, 30000);

			this.unlisten.push(
				await listen('rs2js', async (event) => {
					const cm = event.payload as ChannelMessage;
					console.log("rs2js:", cm);

					const idx = this.requests.findIndex((el) => el.id === cm.id);

					if (cm.state === "Disconnected") {
						this.toDelete.push({
							id: cm.id,
							triggered: new Date().getTime()
						});
					}

					if (idx !== -1) {
						const prev = this.requests.at(idx);
						// Update the existing message at index 'idx'
						this.requests.splice(idx, 1, {
							...cm,
							state: cm.state ?? prev!.state,
							meta: cm.meta ?? prev!.meta,
						});
					} else {
						// Push the new message if not found
						this.requests.push(cm);
					}

					return;
				})
			);

			this.unlisten.push(
				await listen('rs2js_discovery', (event) => {
					const ei = event.payload as EndpointInfo;
					console.log("rs2js:", ei);

					const idx = this.endpointsInfo.findIndex((el) => el.id === ei.id);
					if (!ei.present) {
						if (idx !== -1) {
							this.endpointsInfo.splice(idx, 1);
						}

						return;
					}

					if (idx !== -1) {
						this.endpointsInfo.splice(idx, 1, ei);
					} else {
						this.endpointsInfo.push(ei);
					}
				})
			);

			this.unlisten.push(
				await getCurrent().onFileDropEvent(async (event) => {
					if (event.payload.type === 'hover') {
						this.isDragHovering = true;
					} else if (event.payload.type === 'drop') {
						console.log("Dropped");
						this.isDragHovering = false;
						this.outboundPayload = {
							Files: event.payload.paths
						} as OutboundPayload;
						if (!this.discoveryRunning) await invoke('start_discovery');
						this.discoveryRunning = true;
					} else {
						this.isDragHovering = false;
					}
				})
			);
		});
	},

	unmounted: function() {
		this.unlisten.forEach((el) => el());

		if (this.cleanupInterval && this.cleanupInterval[Symbol.dispose]) {
			this.cleanupInterval[Symbol.dispose]();
		}
	},

	computed: {
		displayedIsEmpty(): boolean {
			return this.displayedItems.length == 0
		},
		displayedItems(): Array<DisplayedItem> {
			const ndisplayed = new Array<DisplayedItem>();

			this.endpointsInfo.forEach((el) => {
				const idx = ndisplayed.findIndex((nel) => el.id == nel.id);
				if (idx !== -1) return;

				ndisplayed.push({
					id: el.id,
					name: el.name ?? 'Unknown',
					deviceType: el.rtype ?? 'Unknown',
					endpoint: true,
				})
			});

			this.requests.filter((el) => stateToDisplay.includes(el.state ?? 'Initial')).forEach((el) => {
				const idx = ndisplayed.findIndex((nel) => el.id == nel.id);
				const elem: DisplayedItem = {
					id: el.id,
					name: el.meta?.source?.name ?? 'Unknown',
					deviceType: el.meta?.source?.device_type ?? 'Unknown',
					endpoint: false,

					state: el.state ?? undefined,
					pin_code: el.meta?.pin_code ?? undefined,
					destination: el.meta?.destination ?? undefined,
					files: el.meta?.files ?? undefined,
					text_description: el.meta?.text_description ?? undefined,
				};

				if (idx !== -1) {
					ndisplayed.splice(idx, 1, elem);
				} else {
					ndisplayed.push(elem)
				}
			});

			return ndisplayed;
		}
	},

	methods: {
		setAutostart: async function(autostart: boolean) {
			if (autostart) {
				await enable();
			} else {
				await disable();
			}

			await this.store.set(autostartKey, autostart);
			await this.store.save();
			this.autostart = autostart;
		},
		applyAutostart: async function() {
			this.autostart = await this.store.get(autostartKey) ?? false;

			if (this.autostart) {
				await enable();
			} else {
				await disable();
			}
		},
		clearSending: async function() {
			await invoke('stop_discovery');
			this.outboundPayload = undefined;
			this.discoveryRunning = false;
			this.endpointsInfo = [];
		},
		removeRequest: function(id: string) {
			const idx = this.requests.findIndex((el) => el.id === id);

			if (idx !== -1) {
				this.requests.splice(idx, 1);
			}
		},
		sendInfo: async function(eid: string) {
			if (this.outboundPayload === undefined) return;

			const ei = this.endpointsInfo.find((el) => el.id === eid);
			if (!ei) return;

			const msg: SendInfo = {
				id: ei.id,
				name: ei.name ?? 'Unknown',
				addr: ei.ip + ":" + ei.port,
				ob: this.outboundPayload,
			};

			await invoke('send_payload', { message: msg });
		},
		sendCmd: async function(id: string, action: ChannelAction) {
			const cm: ChannelMessage = {
				id: id,
				direction: 'FrontToLib',
				action: action,
				meta: null,
				state: null,
				rtype: null,
			};
			console.log("js2rs:", cm);

			await invoke('js2rs', { message: cm });
		}
	},
}
</script>