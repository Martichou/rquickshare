<script setup lang="ts">
import { PropType } from 'vue';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';

defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	},
	openUrl: {
		type: Function as PropType<(url: string) => void>,
		required: true
	}
});

const emit = defineEmits(['openSettings']);
</script>

<template>
	<div class="flex flex-row justify-between items-center px-6 py-4">
		<!-- Header, Pc name left and options right -->
		<div>
			<h4 class="text-md">
				Device name
			</h4>
			<h2 class="text-2xl font-medium">
				{{ vm.hostname }}
			</h2>
		</div>
		<div class="flex justify-center items-center gap-4">
			<div
				class="flex items-center gap-2 text-sm transition duration-150 ease-in-out"
				:class="{'btn active:scale-95': vm.new_version}"
				@click="vm.new_version && openUrl('https://github.com/Martichou/rquickshare/releases/latest')">
				<span v-if="vm.new_version">Update available</span>
				<p>
					v{{ vm.version }}
				</p>
				<p v-if="vm.new_version" class="text-lg">
					→
				</p>
				<p v-if="vm.new_version">
					v{{ vm.new_version }}
				</p>
			</div>
			<div class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out" @click="emit('openSettings')">
				<svg
					xmlns="http://www.w3.org/2000/svg" height="24"
					viewBox="0 -960 960 960" width="24" class="fill-current">
					<!-- eslint-disable-next-line -->
						<path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm70-80h79l14-106q31-8 57.5-23.5T639-327l99 41 39-68-86-65q5-14 7-29.5t2-31.5q0-16-2-31.5t-7-29.5l86-65-39-68-99 42q-22-23-48.5-38.5T533-694l-13-106h-79l-14 106q-31 8-57.5 23.5T321-633l-99-41-39 68 86 64q-5 15-7 30t-2 32q0 16 2 31t7 30l-86 65 39 68 99-42q22 23 48.5 38.5T427-266l13 106Zm42-180q58 0 99-41t41-99q0-58-41-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Zm-2-140Z"/>
				</svg>
			</div>
		</div>
	</div>
</template>