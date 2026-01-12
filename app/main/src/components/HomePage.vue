<template>
	<div class="flex flex-col w-full h-full bg-green-50 dark:bg-slate-900 max-w-full max-h-full overflow-hidden">
		<ToastNotification />
		<SettingsModal :vm="vm" @close="settingsOpen = false" />

		<Heading :vm="vm" :open-url="openUrl" @open-settings="settingsOpen = true" />

		<div class="flex-1 flex flex-row">
			<SideMenu :vm="vm" @invert-visibility="invertVisibility(vm)" @clear-sending="clearSending(vm)" />

			<div class="flex-1 flex flex-col bg-white dark:bg-slate-800 w-full max-w-full min-w-0 min-h-full rounded-tl-[3rem] p-12 h-1 overflow-y-scroll">
				<ContentStatus :vm="vm" @outbound-payload="(el: OutboundPayload) => outboundPayload = el" @discovery-running="discoveryRunning = true;" />

				<div
					v-for="item in displayedItems" :key="item.id" class="w-full rounded-3xl flex flex-row gap-6 p-4 mb-4 bg-green-100 dark:bg-slate-700"
					:class="{'cursor-pointer': item.endpoint}" @click="item.endpoint && sendInfo(vm, item.id)">
					<!-- Loader and image of the device type & pin_code -->
					<ItemSide :item="item" />

					<!-- Content and state of the transfer -->
					<div class="flex-1 flex flex-col text-sm min-w-0" :class="{'justify-center': item.state === undefined}">
						<h4 class="text-base font-medium">
							{{ item.name }}
						</h4>

						<div v-if="item.state === 'WaitingForUserConsent'" class="flex-1 flex flex-col justify-between">
							<p class="mt-4">
								Wants to share {{ item.files?.join(', ') ?? item.text_description ?? 'some file(s).' }}
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click.stop="sendCmd(vm, item.id, 'AcceptTransfer')" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Accept
								</p>
								<p
									@click.stop="sendCmd(vm, item.id, 'RejectTransfer')" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
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
									@click.stop="sendCmd(vm, item.id, 'CancelTransfer')" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Cancel
								</p>
							</div>
						</div>

						<div v-else-if="item.state === 'Finished'">
							<p class="mt-2">
								Received <span v-if="item.text_type">text</span>
							</p>

							<!-- If files -->
							<p v-for="f in item.files ?? []" :key="f" class="overflow-hidden whitespace-nowrap text-ellipsis">
								{{ f }}
							</p>
							<p v-if="item.files" class="mt-2 overflow-hidden whitespace-nowrap text-ellipsis">
								<span v-if="item.files">Saved to </span>{{ item.destination }}
							</p>

							<!-- If text -->
							<p v-if="item.text_type" class="!select-text cursor-text overflow-hidden whitespace-nowrap text-ellipsis">
								{{ item.text_payload }}
							</p>

							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									v-if="item.destination || (item.text_type === 'Url' && item.text_payload)"
									@click.stop="openUrl(item.destination ?? item.text_payload!)"
									class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Open
								</p>
								<p
									v-if="item.text_type && item.text_payload" @click.stop="writeToClipboard(item.text_payload)"
									class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
									Copy
								</p>
								<p
									@click.stop="removeRequest(vm, item.id)"
									class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
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
									@click.stop="removeRequest(vm, item.id)" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
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
									@click.stop="removeRequest(vm, item.id)" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
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
									@click.stop="removeRequest(vm, item.id)" class="btn px-3
									rounded-xl active:scale-95 transition duration-150 ease-in-out shadow-none">
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
import { invoke } from '@tauri-apps/api/core'
import { getVersion } from '@tauri-apps/api/app';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { getStore } from "@tauri-apps/plugin-store";
import { isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';
import { disable, enable } from '@tauri-apps/plugin-autostart';
import { open as tauriDialog } from '@tauri-apps/plugin-dialog';
import { open } from '@tauri-apps/plugin-shell';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

import { ChannelMessage } from '@martichou/core_lib/bindings/ChannelMessage';
import { EndpointInfo } from '@martichou/core_lib/bindings/EndpointInfo';
import { OutboundPayload } from '@martichou/core_lib/bindings/OutboundPayload';
import { Visibility } from '@martichou/core_lib/bindings/Visibility';

import { ToastNotification, ToDelete, stateToDisplay, autostartKey, DisplayedItem, useToastStore, opt, ToastType, utils, Theme } from '../vue_lib';

import SettingsModal from '../composables/SettingsModal.vue';
import Heading from '../composables/Heading.vue';
import SideMenu from '../composables/SideMenu.vue';
import ContentStatus from '../composables/ContentStatus.vue';
import ItemSide from '../composables/ItemSide.vue';

export default {
	name: "HomePage",

	components: {
		ToastNotification,
		SettingsModal,
		Heading,
		SideMenu,
		ContentStatus,
		ItemSide
	},

	async setup() {
		const store = (await getStore(".settings.json"))!;
		const toastStore = useToastStore();

		const dialogOpen = tauriDialog;

		return {
			stateToDisplay,
			store,
			toastStore,
			invoke,
			getVersion,
			enable,
			disable,
			dialogOpen,
			...utils
		};
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

			// eslint-disable-next-line no-undef
			cleanupInterval: opt<NodeJS.Timeout>(),
			unlisten: Array<UnlistenFn>(),

			version: opt<string>(),

			autostart: ref<boolean>(true),
			realclose: ref<boolean>(false),
			startminimized: ref<boolean>(false),
			visibility: ref<Visibility>('Visible'),
			theme: ref<Theme>('System'),
			downloadPath: ref<string | undefined>(),

			hostname: ref<string>(),

			settingsOpen: ref<boolean>(false),

			new_version: opt<string>(),
		};
	},

	mounted: function () {
		nextTick(async () => {
			this.hostname = await invoke('get_hostname');
			this.version = await getVersion();

			await this.getVisibility(this);
			await this.getTheme(this);

			if (!await this.store.has(autostartKey)) {
				await this.setAutoStart(this, true);
			} else {
				await this.applyAutoStart(this);
			}

			await this.getRealclose(this);
			await this.getStartMinimized(this);
			await this.getDownloadPath(this);

			// Check permission for notification
			let permissionGranted = await isPermissionGranted();
			if (!permissionGranted) {
				const permission = await requestPermission();
				permissionGranted = permission === 'granted';
			}

			this.unlisten.push(
				await listen('rs2js_channelmessage', async (event) => {
					const cm = event.payload as ChannelMessage;
					const idx = this.requests.findIndex((el) => el.id === cm.id);

					if (cm.state === "Disconnected") {
						this.toDelete.push({
							id: cm.id,
							triggered: new Date().getTime()
						});
					}

					// TODO - Automatically open || copy to clipboard + toast

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
				await listen('rs2js_endpointinfo', (event) => {
					const ei = event.payload as EndpointInfo;
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
				await listen('visibility_updated', async () => {
					console.log("Visibility changed");
					await this.getVisibility(this);
				})
			);

			this.unlisten.push(
				await getCurrentWindow().onDragDropEvent(async (event) => {
					if (event.payload.type === 'over') {
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

			await this.getLatestVersion(this);
		});
	},

	unmounted: function() {
		this.unlisten.forEach((el) => el());

		if (this.cleanupInterval && this.cleanupInterval[Symbol.dispose]) {
			this.cleanupInterval[Symbol.dispose]();
		}
	},

	computed: {
		vm() {
			return this;
		},
		displayedIsEmpty(): boolean {
			return this.displayedItems.length == 0
		},
		displayedItems(): Array<DisplayedItem> {
			return this._displayedItems(this);
		}
	},

	methods: {
		writeToClipboard: async function(text: string) {
			try {
				await writeText(text);
				this.toastStore.addToast("Copied to clipboard", ToastType.Success);
			} catch (e) {
				this.toastStore.addToast("Unknown error while copying text", ToastType.Error);
				console.error("Error copying text", e);
			}
		},
		openUrl: async function(url: string) {
			try {
				await open(url);
			} catch (e) {
				this.toastStore.addToast("Error opening URL, it may not be a valid URI", ToastType.Error);
				console.error("Error opening URL", e);
			}
		},
	},
}
</script>