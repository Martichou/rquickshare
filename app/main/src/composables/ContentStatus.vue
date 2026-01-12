<script setup lang="ts">
import { OutboundPayload } from '@martichou/core_lib/bindings/OutboundPayload';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';
import { PropType } from 'vue';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emits = defineEmits(['outboundPayload', 'discoveryRunning']);

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
        rounded-2xl p-6 flex flex-col justify-center items-center transition duration-150 ease-in-out mt-auto dark:border-slate-600"
		:class="{'border-green-200 dark:border-green-700 bg-green-100 dark:bg-slate-700 scale-105': props.vm.isDragHovering}">
		<svg
			xmlns="http://www.w3.org/2000/svg" height="24"
			viewBox="0 -960 960 960" width="24" class="w-8 h-8 fill-current">
			<!-- eslint-disable-next-line -->
            <path d="M440-320v-326L336-542l-56-58 200-200 200 200-56 58-104-104v326h-80ZM240-160q-33 0-56.5-23.5T160-240v-120h80v120h480v-120h80v120q0 33-23.5 56.5T720-160H240Z" />
		</svg>
		<h4 class="mt-2 font-medium">
			Drop files to send
		</h4>
		<div class="btn mt-2 active:scale-95 transition duration-150 ease-in-out" @click="openFilePicker()">
			<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" class="fill-current">
				<path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z" />
			</svg>
			<span class="ml-2">Select</span>
		</div>
	</div>
</template>