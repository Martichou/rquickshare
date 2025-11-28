<script setup lang="ts">
import { PropType } from 'vue';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';
import { utils } from '../vue_lib';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emit = defineEmits(['trust', 'block']);

async function handleTrust() {
	if (!props.vm.pendingTrustRequest) return;
	const { name, deviceType } = props.vm.pendingTrustRequest;
	await utils.addTrustedDevice(props.vm, name, deviceType);
	emit('trust');
}

async function handleBlock() {
	if (!props.vm.pendingTrustRequest) return;
	const { name, deviceType } = props.vm.pendingTrustRequest;
	await utils.addBlockedDevice(props.vm, name, deviceType);
	emit('block');
}
</script>

<template>
	<div
		v-if="vm.pendingTrustRequest"
		class="bg-blue-50 border border-blue-200 rounded-xl p-4 mb-4 shadow-sm"
	>
		<div class="flex flex-col gap-3">
			<div class="flex items-start gap-3">
				<div class="flex-shrink-0 w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
					<svg class="w-4 h-4 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
					</svg>
				</div>
				<div class="flex-1">
					<h4 class="font-medium text-blue-900">
						Trust "{{ vm.pendingTrustRequest.name }}"?
					</h4>
					<p class="text-sm text-blue-700 mt-1">
						Auto-accept incoming transfers from this device when the last transfer was within {{ vm.autoAcceptTimeout }} {{ vm.autoAcceptTimeout === 1 ? 'minute' : 'minutes' }}.
					</p>
				</div>
			</div>
			<div class="flex flex-row justify-end gap-2 flex-wrap">
				<button
					@click="handleTrust"
					class="btn px-3 py-1 rounded-xl bg-blue-600 text-white hover:bg-blue-700 active:scale-95 transition duration-150 ease-in-out shadow-none text-sm"
				>
					Yes, trust
				</button>
				<button
					@click="handleBlock"
					class="btn px-3 py-1 rounded-xl bg-gray-100 text-gray-600 hover:bg-gray-200 active:scale-95 transition duration-150 ease-in-out shadow-none text-sm"
				>
					Don't ask again
				</button>
			</div>
		</div>
	</div>
</template>
