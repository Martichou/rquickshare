<script setup lang="ts">
import { PropType, ref, watch, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';

interface DeviceNameValidation {
	valid: boolean;
	error: string | null;
}

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	},
	openUrl: {
		type: Function as PropType<(url: string) => void>,
		required: true
	}
});

const emit = defineEmits(['openSettings', 'hostnameUpdated']);

const isEditing = ref(false);
const editedName = ref('');
const validationError = ref<string | null>(null);
const isSaving = ref(false);
const inputRef = ref<HTMLInputElement | null>(null);
const measureRef = ref<HTMLSpanElement | null>(null);
const inputWidth = ref('auto');

const updateInputWidth = () => {
	nextTick(() => {
		if (measureRef.value) {
			const width = measureRef.value.offsetWidth;
			inputWidth.value = `${Math.max(width + 20, 100)}px`;
		}
	});
};

watch(() => props.vm.hostname, (newVal) => {
	if (!isEditing.value && newVal) {
		editedName.value = newVal;
	}
});

watch(editedName, () => {
	updateInputWidth();
});

const startEditing = () => {
	editedName.value = props.vm.hostname || '';
	validationError.value = null;
	isEditing.value = true;
	nextTick(() => {
		updateInputWidth();
		inputRef.value?.focus();
		inputRef.value?.select();
	});
};

const cancelEditing = () => {
	isEditing.value = false;
	editedName.value = props.vm.hostname || '';
	validationError.value = null;
};

const validateName = async () => {
	if (!editedName.value.trim()) {
		validationError.value = null;
		return;
	}
	try {
		const result = await invoke<DeviceNameValidation>('validate_device_name_cmd', {
			name: editedName.value
		});
		validationError.value = result.error;
	} catch {
		validationError.value = 'Validation error';
	}
};

const saveName = async () => {
	if (isSaving.value) return;

	const nameToSave = editedName.value.trim() || null;

	isSaving.value = true;
	try {
		const result = await invoke<DeviceNameValidation>('set_device_name', {
			name: nameToSave
		});

		if (result.valid) {
			isEditing.value = false;
			validationError.value = null;
			emit('hostnameUpdated');
		} else {
			validationError.value = result.error;
		}
	} catch (e) {
		validationError.value = 'Failed to save device name';
		console.error('Error saving device name:', e);
	} finally {
		isSaving.value = false;
	}
};

const handleKeydown = (e: KeyboardEvent) => {
	if (e.key === 'Enter') {
		saveName();
	} else if (e.key === 'Escape') {
		cancelEditing();
	}
};
</script>

<template>
	<div class="flex flex-row justify-between items-center px-6 py-4">
		<!-- Header, Pc name left and options right -->
		<div class="min-w-0 flex-1 mr-4">
			<h4 class="text-md">
				Device name
			</h4>
			<div v-if="isEditing" class="flex flex-col gap-1">
				<div class="flex items-center gap-2">
					<!-- Hidden span to measure text width -->
					<span
						ref="measureRef"
						class="text-2xl font-medium px-2 py-1 absolute invisible whitespace-pre"
						aria-hidden="true">{{ editedName || 'Enter device name...' }}</span>
					<input
						ref="inputRef"
						v-model="editedName"
						type="text"
						class="text-2xl font-medium bg-white border-2 rounded-lg px-2 py-1 outline-none transition-all duration-150"
						:class="validationError ? 'border-red-400 focus:border-red-500' : 'border-green-300 focus:border-green-500'"
						:style="{ width: inputWidth }"
						placeholder="Enter device name..."
						maxlength="255"
						@input="validateName"
						@keydown="handleKeydown"
						@blur="validateName">
					<button
						class="btn px-3 py-1 rounded-lg active:scale-95 transition duration-150 ease-in-out text-sm"
						:disabled="isSaving || !!validationError"
						@click="saveName">
						{{ isSaving ? '...' : 'Save' }}
					</button>
					<button
						class="btn px-3 py-1 rounded-lg active:scale-95 transition duration-150 ease-in-out text-sm"
						:disabled="isSaving"
						@click="cancelEditing">
						Cancel
					</button>
				</div>
				<p v-if="validationError" class="text-red-500 text-sm">
					{{ validationError }}
				</p>
				<p v-else class="text-gray-400 text-xs">
					Leave empty to use system hostname
				</p>
			</div>
			<h2
				v-else
				class="text-2xl font-medium cursor-pointer hover:text-green-700 transition-colors group flex items-center gap-2"
				title="Click to edit device name"
				@click="startEditing">
				<span class="truncate">{{ vm.hostname }}</span>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="w-4 h-4 opacity-0 group-hover:opacity-100 transition-opacity flex-shrink-0"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2">
					<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
					<path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
				</svg>
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
					viewBox="0 -960 960 960" width="24">
					<!-- eslint-disable-next-line -->
						<path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm70-80h79l14-106q31-8 57.5-23.5T639-327l99 41 39-68-86-65q5-14 7-29.5t2-31.5q0-16-2-31.5t-7-29.5l86-65-39-68-99 42q-22-23-48.5-38.5T533-694l-13-106h-79l-14 106q-31 8-57.5 23.5T321-633l-99-41-39 68 86 64q-5 15-7 30t-2 32q0 16 2 31t7 30l-86 65 39 68 99-42q22 23 48.5 38.5T427-266l13 106Zm42-180q58 0 99-41t41-99q0-58-41-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Zm-2-140Z"/>
				</svg>
			</div>
		</div>
	</div>
</template>