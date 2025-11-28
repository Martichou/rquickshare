<script setup lang="ts">
import { utils, autoAcceptTimeoutOptions, AutoAcceptTimeout, TrustedDevice } from '../vue_lib';
import { PropType, computed, ref, watch, onUnmounted } from 'vue';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emit = defineEmits(['close']);

// Timer for countdown updates
let countdownInterval: ReturnType<typeof setInterval> | null = null;
const now = ref(Date.now());

// Watch for settings modal opening/closing
watch(() => props.vm.settingsOpen, async (isOpen) => {
	if (isOpen) {
		// Refresh trusted devices when opening settings (removes expired ones)
		await utils.getTrustedDevices(props.vm);
		// Start countdown timer
		now.value = Date.now();
		countdownInterval = setInterval(() => {
			now.value = Date.now();
			// Also check for expired devices and remove them
			checkAndRemoveExpired();
		}, 1000);
	} else {
		// Stop timer when closing
		if (countdownInterval) {
			clearInterval(countdownInterval);
			countdownInterval = null;
		}
	}
});

onUnmounted(() => {
	if (countdownInterval) {
		clearInterval(countdownInterval);
	}
});

async function checkAndRemoveExpired() {
	const timeoutMs = props.vm.autoAcceptTimeout * 60 * 1000;
	const expired = props.vm.trustedDevices.filter(d => (now.value - d.lastTransfer) >= timeoutMs);
	for (const device of expired) {
		await utils.removeTrustedDevice(props.vm, device.name);
	}
}

function getTimeRemaining(device: TrustedDevice): string {
	const timeoutMs = props.vm.autoAcceptTimeout * 60 * 1000;
	const elapsed = now.value - device.lastTransfer;
	const remaining = timeoutMs - elapsed;

	if (remaining <= 0) return 'Expired';

	const totalSeconds = Math.floor(remaining / 1000);
	const minutes = Math.floor(totalSeconds / 60);
	const seconds = totalSeconds % 60;

	if (minutes > 0) {
		return `${minutes}m ${seconds}s`;
	}
	return `${seconds}s`;
}

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

function onAutoAcceptTimeoutChange(event: Event) {
	const target = event.target as HTMLSelectElement;
	const value = parseInt(target.value) as AutoAcceptTimeout;
	utils.setAutoAcceptTimeout(props.vm, value);
}

async function removeTrusted(name: string) {
	await utils.removeTrustedDevice(props.vm, name);
}

async function removeBlocked(name: string) {
	await utils.removeBlockedDevice(props.vm, name);
}

const hasDevices = computed(() => {
	return props.vm.trustedDevices.length > 0 || props.vm.blockedDevices.length > 0;
});
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
				<div class="form-control hover:bg-gray-500 hover:bg-opacity-10 rounded-xl p-3">
					<label class="flex flex-row justify-between items-center">
						<span class="label-text">Auto-accept timeout</span>
						<select
							class="select select-sm bg-gray-100 focus:outline-none"
							:value="vm.autoAcceptTimeout"
							@change="onAutoAcceptTimeoutChange">
							<option
								v-for="option in autoAcceptTimeoutOptions"
								:key="option.value"
								:value="option.value">
								{{ option.label }}
							</option>
						</select>
					</label>
				</div>

				<!-- Devices Section -->
				<div v-if="hasDevices" class="mt-4 pt-4 border-t border-gray-200">
					<h4 class="font-medium text-sm text-gray-700 mb-2">Devices</h4>

					<!-- Trusted Devices -->
					<div v-if="vm.trustedDevices.length > 0" class="mb-3">
						<p class="text-xs text-gray-500 mb-1">Trusted (auto-accept enabled)</p>
						<div
							v-for="device in vm.trustedDevices"
							:key="device.name"
							class="flex flex-row justify-between items-center py-2 px-3 hover:bg-gray-50 rounded-lg"
						>
							<div class="flex flex-col min-w-0 flex-1">
								<span class="text-sm truncate">{{ device.name }}</span>
								<span class="text-xs text-gray-400">Expires in {{ getTimeRemaining(device) }}</span>
							</div>
							<button
								@click="removeTrusted(device.name)"
								class="text-xs text-red-500 hover:text-red-700 ml-2 flex-shrink-0"
							>
								Remove
							</button>
						</div>
					</div>

					<!-- Blocked Devices -->
					<div v-if="vm.blockedDevices.length > 0">
						<p class="text-xs text-gray-500 mb-1">Blocked (won't ask to trust)</p>
						<div
							v-for="device in vm.blockedDevices"
							:key="device.name"
							class="flex flex-row justify-between items-center py-2 px-3 hover:bg-gray-50 rounded-lg"
						>
							<span class="text-sm truncate flex-1">{{ device.name }}</span>
							<button
								@click="removeBlocked(device.name)"
								class="text-xs text-blue-500 hover:text-blue-700 ml-2"
							>
								Unblock
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>