<script setup lang="ts">
import { ref } from 'vue'
// import { listen } from '@tauri-apps/api/event'
// import { invoke } from '@tauri-apps/api/tauri'

enum FileType {
	UNKNOWN = 0,
	IMAGE = 1,
	VIDEO = 2,
	APP = 3,
	AUDIO = 4,
}

type FileMetadata = {
	name?: string;
	type?: FileType;
	payload_id?: number;
	size?: number;
	mime_type?: string;
	id?: number;
}

interface TransferMetadata {
	id: number;
	files: Array<string>;
	pin_code?: string;
	description?: string;
}

type InternalFileInfo = {
	meta: FileMetadata;
	payload_id: number;
	destination_url: string;
	bytes_transferred: number;
}

interface TransferRequest {
	name: string;
	device_type: number;
	state: number;
	transfer_metadata?: TransferMetadata;
	transferred_files: Record<number, InternalFileInfo>;
}

const requests = ref<TransferRequest[]>([{
	name: "Martin's Phone",
	device_type: 1,
	state: 7,
	transfer_metadata: {
		id: 1,
		files: ["test1.png", "test2.png"],
		pin_code: "1234"
	},
	transferred_files: []
}]);

// function sendOutput() {
//   invoke('js2rs', { message: "" })
// }

// await listen('rs2js', (event) => {
//   console.log("js: rs2js: " + event)
// })
</script>

<template>
	<div class="prose prose-sm flex flex-col bg-green-50 w-full h-full max-w-full max-h-full">
		<div class="flex flex-row justify-between items-center px-6 py-4">
			<!-- Header, Pc name left and options right -->
			<div>
				<h4 class="py-0 my-0 font-normal">
					Device name
				</h4>
				<h3 class="py-0 my-0">
					Rtin
				</h3>
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
				<p class="mt-4 mb-1">
					Currently
				</p>
				<h4 class="mt-0">
					Receiving from everyone
				</h4>
				<p>Everyone can share with you (you still need to approve each transfer).</p>
			</div>
			<div
				class="flex-1 flex flex-col h-full rounded-tl-[3rem] bg-white p-6"
				:class="{'items-center': requests.filter((el) => el.state >= 7).length === 0}">
				<h3 class="my-4">
					Ready to receive
				</h3>

				<div v-if="requests.filter((el) => el.state >= 7).length === 0" class="my-auto status-indicator status-indicator--success status-indicator--xl">
					<div class="circle circle--animated circle-main" />
					<div class="circle circle--animated circle-secondary" />
					<div class="circle circle--animated circle-tertiary" />
				</div>

				<div
					v-for="request in requests.filter((el) => el.state >= 7)"
					:key="request.name"
					class="bg-green-200 rounded-3xl flex flex-row gap-6 p-6">
					<div>
						<div class="h-16 w-16 rounded-full bg-green-50">
							<svg
								v-if="request.device_type === 1" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4 fill-gray-900">
								<!-- eslint-disable-next-line -->
								<path d="M0-160v-80h160v-40q-33 0-56.5-23.5T80-360v-400q0-33 23.5-56.5T160-840h640q33 0 56.5 23.5T880-760v400q0 33-23.5 56.5T800-280v40h160v80H0Zm160-200h640v-400H160v400Zm0 0v-400 400Z" />
							</svg>
							<svg
								v-else-if="request.device_type === 2" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
								width="24" class="w-full h-full p-4 fill-gray-900">
								<!-- eslint-disable-next-line -->
								<path d="M280-40q-33 0-56.5-23.5T200-120v-720q0-33 23.5-56.5T280-920h400q33 0 56.5 23.5T760-840v720q0 33-23.5 56.5T680-40H280Zm0-120v40h400v-40H280Zm0-80h400v-480H280v480Zm0-560h400v-40H280v40Zm0 0v-40 40Zm0 640v40-40Z" />
							</svg>
							<svg
								v-else-if="request.device_type === 3" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
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
						<p v-if="request.state === 7" class="text-center inline-flex gap-1 mb-0 text-sm items-center">
							<svg
								xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
								class="fill-gray-900">
								<!-- eslint-disable-next-line -->
								<path d="M420-360h120l-23-129q20-10 31.5-29t11.5-42q0-33-23.5-56.5T480-640q-33 0-56.5 23.5T400-560q0 23 11.5 42t31.5 29l-23 129Zm60 280q-139-35-229.5-159.5T160-516v-244l320-120 320 120v244q0 152-90.5 276.5T480-80Zm0-84q104-33 172-132t68-220v-189l-240-90-240 90v189q0 121 68 220t172 132Zm0-316Z" />
							</svg>
							{{ request.transfer_metadata?.pin_code }}
						</p>
					</div>
					<div class="flex-1 flex flex-col">
						<h4 class="mt-0">
							Martin's Phone
						</h4>
						<div v-if="request.state === 7" class="flex-1 flex flex-col justify-between">
							<p class="mt-2 mb-0">
								Wants to share {{ request.transfer_metadata?.files.join(', ') ?? 'some file(s).' }}
							</p>
							<div class="flex flex-row justify-end gap-4 mt-1">
								<p
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-100 rounded-lg
									font-semibold active:scale-105 transition duration-150 ease-in-out">
									Accept
								</p>
								<p
									class="my-0 cursor-pointer p-2 px-3 hover:bg-green-100 rounded-lg
									font-semibold active:scale-105 transition duration-150 ease-in-out">
									Decline
								</p>
							</div>
						</div>
						<div v-else-if="request.state === 8">
							<p class="mt-2 font-medium">
								Receiving...
							</p>
							<p>{{ request.transferred_files }}</p>
							<p />
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>