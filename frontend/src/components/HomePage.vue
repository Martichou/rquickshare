<script setup lang="ts">
import { ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification'

import { ChannelMessage } from '../../../core_lib/bindings/ChannelMessage';
import { ChannelAction } from '../../../core_lib/bindings/ChannelAction';

let isAppInForeground = false;
// Do you have permission to send a notification?
let permissionGranted = await isPermissionGranted();

// If not we need to request it
if (!permissionGranted) {
	const permission = await requestPermission();
	permissionGranted = permission === 'granted';
}

const _stateToDisplay = ["ReceivedPairedKeyResult", "WaitingForUserConsent", "ReceivingFiles", "Disconnected", "Finished"]

interface ToDelete {
	id: string,
	triggered: number
}

const requests = ref<ChannelMessage[]>([]);
const toDelete = ref<ToDelete[]>([]);
const requestIsEmpty = computed(() => {
	return requests.value.filter((el) => _stateToDisplay.includes(el.state ?? 'Initial')).length == 0
});

async function sendCmd(id: string, action: ChannelAction) {
	const cm: ChannelMessage = {
		id: id,
		direction: 'FrontToLib',
		action: action,
		meta: null,
		state: null,
	};
	console.log("js2rs:", cm);

	await invoke('js2rs', { message: cm });
}

function removeRequest(id: string) {
	const idx = requests.value.findIndex((el) => el.id === id);

	if (idx !== -1) {
		requests.value.splice(idx, 1);
	}
}

await listen('rs2js', (event) => {
	const cm = event.payload as ChannelMessage;
	console.log("rs2js:", cm);

	const idx = requests.value.findIndex((el) => el.id === cm.id);

	if (cm.state === "Disconnected") {
		toDelete.value.push({
			id: cm.id,
			triggered: new Date().getTime()
		});
	}

	if (idx != -1) {
		const prev = requests.value.at(idx);
		// Update the existing message at index 'idx'
		requests.value.splice(idx, 1, {
			...cm,
			state: cm.state ?? prev!.state,
			meta: cm.meta ?? prev!.meta,
		});
	} else {
		if (isAppInForeground && permissionGranted && cm.state === 'WaitingForUserConsent') {
			sendNotification({ title: 'New transfer request', body: (cm.meta?.source?.name ?? 'Unknown') + ' want to initiate a transfer.' });
		}

		// Push the new message if not found
		requests.value.push(cm);
	}
})

setInterval(() => {
	toDelete.value.forEach((itemToDelete) => {
		const now = new Date();
		const timeDifference = now.getTime() - itemToDelete.triggered;

		// Check if at least 30 seconds have passed (30000 milliseconds)
		if (timeDifference >= 30000) removeRequest(itemToDelete.id);
	});

	// Clear only elements that have been processed (more than 30s old)
	toDelete.value = toDelete.value.filter((item) => {
		const now = new Date();
		return now.getTime() - item.triggered < 30000;
	});
}, 30000);

window.addEventListener('focus', () => {
	isAppInForeground = true;
});

window.addEventListener('blur', () => {
	isAppInForeground = false;
});
</script>

<template>
	<div class="flex flex-col bg-green-50 bg-opacity-75 w-full h-full max-w-full max-h-full">
		<div class="flex flex-row justify-between items-center px-6 py-4">
			<!-- Header, Pc name left and options right -->
			<div>
				<h4 class="text-md">
					Device name
				</h4>
				<h2 class="text-2xl font-medium">
					Rtin
				</h2>
			</div>
			<div>
				<div class="hover:bg-gray-200 cursor-pointer p-2 rounded-lg active:scale-105 transition duration-150 ease-in-out">
					<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24">
						<!-- eslint-disable-next-line -->
						<path d="M480-160q-33 0-56.5-23.5T400-240q0-33 23.5-56.5T480-320q33 0 56.5 23.5T560-240q0 33-23.5 56.5T480-160Zm0-240q-33 0-56.5-23.5T400-480q0-33 23.5-56.5T480-560q33 0 56.5 23.5T560-480q0 33-23.5 56.5T480-400Zm0-240q-33 0-56.5-23.5T400-720q0-33 23.5-56.5T480-800q33 0 56.5 23.5T560-720q0 33-23.5 56.5T480-640Z" />
					</svg>
				</div>
			</div>
		</div>
		<div class="flex-1 flex flex-row">
			<!-- Content -->
			<!-- When uploading: left info about current file being uploaded -->
			<!-- 				 right, list of nearby devices -->
			<!-- Default: left settings about visibility -->
			<!-- 		  right, ready to receive with hint for drag & drop, then request (to accept or not) -->
			<div class="w-72 p-6">
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
			<div
				class="flex-1 flex flex-col h-full rounded-tl-[3rem] bg-white p-12"
				:class="{'items-center': requestIsEmpty}">
				<h3 class="mb-4 font-medium text-xl">
					<span v-if="requestIsEmpty">Ready to receive</span>
					<span v-else>Nearby devices</span>
				</h3>

				<div v-if="requestIsEmpty" class="my-auto status-indicator status-indicator--success status-indicator--xl">
					<div class="circle circle--animated circle-main" />
					<div class="circle circle--animated circle-secondary" />
					<div class="circle circle--animated circle-tertiary" />
				</div>

				<div
					v-for="request in requests.filter((el) => _stateToDisplay.includes(el.state ?? 'Initial'))"
					:key="request.id"
					class="bg-green-200 bg-opacity-65 rounded-3xl flex flex-row gap-6 p-6 mb-4"
					:class="{'pb-4': ['WaitingForUserConsent', 'Finished'].includes(request.state ?? '')}">
					<div>
						<div class="h-16 w-16 rounded-full bg-green-50" :class="{'!bg-green-400': request.state === 'Finished'}">
							<svg
								v-if="request.state === 'Finished'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4 fill-white">
								<!-- eslint-disable-next-line -->
								<path d="M268-240 42-466l57-56 170 170 56 56-57 56Zm226 0L268-466l56-57 170 170 368-368 56 57-424 424Zm0-226-57-56 198-198 57 56-198 198Z" />
							</svg>
							<svg
								v-else-if="request.meta?.source?.device_type === 'Laptop'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4 fill-gray-900">
								<!-- eslint-disable-next-line -->
								<path d="M0-160v-80h160v-40q-33 0-56.5-23.5T80-360v-400q0-33 23.5-56.5T160-840h640q33 0 56.5 23.5T880-760v400q0 33-23.5 56.5T800-280v40h160v80H0Zm160-200h640v-400H160v400Zm0 0v-400 400Z" />
							</svg>
							<svg
								v-else-if="request.meta?.source?.device_type === 'Phone'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4 fill-gray-900">
								<!-- eslint-disable-next-line -->
								<path d="M280-40q-33 0-56.5-23.5T200-120v-720q0-33 23.5-56.5T280-920h400q33 0 56.5 23.5T760-840v720q0 33-23.5 56.5T680-40H280Zm0-120v40h400v-40H280Zm0-80h400v-480H280v480Zm0-560h400v-40H280v40Zm0 0v-40 40Zm0 640v40-40Z" />
							</svg>
							<svg
								v-else-if="request.meta?.source?.device_type === 'Tablet'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
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
						<p v-if="request.state === 'WaitingForUserConsent'" class="text-center inline-flex gap-1 mt-4 text-sm items-center">
							<svg
								xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
								class="fill-gray-900">
								<!-- eslint-disable-next-line -->
								<path d="M420-360h120l-23-129q20-10 31.5-29t11.5-42q0-33-23.5-56.5T480-640q-33 0-56.5 23.5T400-560q0 23 11.5 42t31.5 29l-23 129Zm60 280q-139-35-229.5-159.5T160-516v-244l320-120 320 120v244q0 152-90.5 276.5T480-80Zm0-84q104-33 172-132t68-220v-189l-240-90-240 90v189q0 121 68 220t172 132Zm0-316Z" />
							</svg>
							{{ request.meta?.pin_code }}
						</p>
					</div>
					<div class="flex-1 flex flex-col text-sm">
						<h4 class="text-base font-medium">
							{{ request.meta?.source?.name ?? 'Unknown' }}
						</h4>
						<div v-if="request.state === 'WaitingForUserConsent'" class="flex-1 flex flex-col justify-between">
							<p class="mt-4">
								Wants to share {{ request.meta?.files?.join(', ') ?? request.meta?.text_description ?? 'some file(s).' }}
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="sendCmd(request.id, 'AcceptTransfer')"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-lg
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Accept
								</p>
								<p
									@click="sendCmd(request.id, 'RejectTransfer')"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-lg
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Decline
								</p>
							</div>
						</div>
						<div v-else-if="request.state === 'ReceivingFiles'">
							<p class="mt-2">
								Receiving...
							</p>
							<p v-for="f in request.meta?.files ?? []" :key="f">
								{{ f }}
							</p>
						</div>
						<div v-else-if="request.state === 'Finished'">
							<p class="mt-2">
								Received
							</p>
							<p v-if="request.meta?.destination !== null">
								Saved to {{ request.meta?.destination }}
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									v-if="request.meta?.destination"
									@click="invoke('open', { message: request.meta?.destination })"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-lg
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Open
								</p>
								<p
									@click="removeRequest(request.id)"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-lg
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Clear
								</p>
							</div>
						</div>
						<div v-else-if="request.state === 'Disconnected'">
							<p class="mt-2">
								Unexpected disconnection
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									@click="removeRequest(request.id)"
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-50 rounded-lg
									font-medium active:scale-105 transition duration-150 ease-in-out">
									Clear
								</p>
							</div>
						</div>
					</div>
					<div v-if="request.state === 'ReceivingFiles'" class="my-auto">
						<div class="hover:bg-gray-200 cursor-pointer p-2 rounded-lg active:scale-105 transition duration-150 ease-in-out">
							<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24">
								<path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z" />
							</svg>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>
