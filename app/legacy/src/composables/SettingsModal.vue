<script setup lang="ts">
import { utils } from '../vue_lib';
import { PropType } from 'vue';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emit = defineEmits(['close']);

function openDownloadPicker() {
	props.vm.dialogOpen({
		title: "Select the destination for files",
		directory: true,
		multiple: false,
	}).then(async (el) => {
		if (el === null) {
			return;
		}

		await utils.setDownloadPath(props.vm, el as string);
	});
}
</script>

<template>
	<div v-if="vm.settingsOpen" class="absolute z-10 w-full h-full flex justify-center items-center bg-black bg-opacity-25">
		<div class="bg-white rounded-xl shadow-xl p-4 w-[24rem]">
			<div class="flex flex-row justify-between items-center">
				<h3 class="font-medium text-xl">
					Settings
				</h3>
				<div class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out" @click="emit('close')">
					Close
				</div>
			</div>
			<div class="py-4 flex flex-col">
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center" @click="utils.setAutoStart(vm, !vm.autostart)">
						<span class="label-text">Start on boot</span>
						<input type="checkbox" :checked="vm.autostart" class="checkbox focus:outline-none">
					</label>
				</div>
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center" @click="utils.setRealClose(vm, !vm.realclose)">
						<span class="label-text">Keep running on close</span>
						<input type="checkbox" :checked="!vm.realclose" class="checkbox focus:outline-none">
					</label>
				</div>
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center" @click="utils.setStartMinimized(vm, !vm.startminimized)">
						<span class="label-text">Start minimized</span>
						<input type="checkbox" :checked="vm.startminimized" class="checkbox focus:outline-none">
					</label>
				</div>
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<label class="cursor-pointer flex flex-col items-start" @click="openDownloadPicker()">
						<span class="">Change download folder</span>
						<span class="overflow-hidden whitespace-nowrap text-ellipsis text-xs max-w-80">
							> {{ vm.downloadPath ?? 'OS User\'s download folder' }}
						</span>
					</label>
				</div>
			</div>
		</div>
	</div>
</template>