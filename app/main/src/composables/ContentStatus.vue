<script setup lang="ts">
import { readText } from '@tauri-apps/plugin-clipboard-manager';
import { OutboundPayload } from '@martichou/core_lib/bindings/OutboundPayload';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';
import { useToastStore, ToastType } from '../vue_lib';
import { PropType } from 'vue';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emits = defineEmits(['outboundPayload', 'discoveryRunning']);
const toastStore = useToastStore();

function openFilePicker() {
	props.vm.dialogOpen({
		title: "Select a file to send",
		directory: false,
		multiple: true,
	}).then(async (el) => {
		let elem;
		if (el === null) {
			return;
		}

		if (el instanceof Array) {
			if (el.length > 0 && Object.hasOwn(el[0], 'path')) {
				elem = el.map((e) => e.path);
			} else {
				elem = el;
			}
		} else {
			elem = [el];
		}

		emits('outboundPayload', {
			Files: elem
		} as OutboundPayload);
		if (!props.vm.discoveryRunning) await props.vm.invoke('start_discovery');
		emits('discoveryRunning');
	})
}

async function shareClipboard() {
	let text: string | null = null;
	try {
		text = await readText();
	} catch {
		// No text — try image clipboard
		try {
			const tempPath = await props.vm.invoke<string>('save_clipboard_image');
			emits('outboundPayload', { Files: [tempPath] } as OutboundPayload);
			if (!props.vm.discoveryRunning) await props.vm.invoke('start_discovery');
			emits('discoveryRunning');
		} catch {
			toastStore.addToast("Clipboard is empty", ToastType.Error);
		}
		return;
	}
	if (!text) {
		toastStore.addToast("Clipboard is empty", ToastType.Error);
		return;
	}
	emits('outboundPayload', { Text: text } as OutboundPayload);
	if (!props.vm.discoveryRunning) await props.vm.invoke('start_discovery');
	emits('discoveryRunning');
}
</script>

<template>
	<h3 class="mb-4 font-medium text-xl">
		<span v-if="props.vm.displayedIsEmpty">Ready to receive{{ props.vm.outboundPayload != undefined ? ' / send' : '' }}</span>
		<span v-else>Nearby devices</span>
	</h3>

	<div v-if="props.vm.displayedIsEmpty && props.vm.endpointsInfo.length === 0" class="m-auto status-indicator status-indicator--success status-indicator--xl">
		<div class="circle circle--animated circle-main" />
		<div class="circle circle--animated circle-secondary" />
		<div class="circle circle--animated circle-tertiary" />
	</div>

	<div
		v-if="props.vm.displayedIsEmpty && props.vm.outboundPayload === undefined" class="w-full border
        rounded-2xl p-6 flex flex-col justify-center items-center transition duration-150 ease-in-out mt-auto"
		:class="{'border-green-200 bg-green-100 scale-105': props.vm.isDragHovering}">
		<svg
			xmlns="http://www.w3.org/2000/svg" height="24"
			viewBox="0 -960 960 960" width="24" class="w-8 h-8">
			<!-- eslint-disable-next-line -->
            <path d="M440-320v-326L336-542l-56-58 200-200 200 200-56 58-104-104v326h-80ZM240-160q-33 0-56.5-23.5T160-240v-120h80v120h480v-120h80v120q0 33-23.5 56.5T720-160H240Z" />
		</svg>
		<h4 class="mt-2 font-medium">
			Drop files to send
		</h4>
		<div class="flex gap-2 mt-2">
			<div class="btn active:scale-95 transition duration-150 ease-in-out" @click="openFilePicker()">
				<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24">
					<path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z" />
				</svg>
				<span class="ml-2">Select</span>
			</div>
			<div class="btn active:scale-95 transition duration-150 ease-in-out" @click="shareClipboard()">
				<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24">
					<path d="M360-240q-33 0-56.5-23.5T280-320v-480q0-33 23.5-56.5T360-880h360q33 0 56.5 23.5T800-800v480q0 33-23.5 56.5T720-240H360Zm0-80h360v-480H360v480ZM200-80q-33 0-56.5-23.5T120-160v-560h80v560h440v80H200Zm160-240v-480 480Z"/>
				</svg>
				<span class="ml-2">Clipboard</span>
			</div>
		</div>
	</div>
</template>