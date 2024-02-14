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

const requests = ref<TransferRequest[]>([{name: "Martin's Phone", device_type: 1, state: 7, transferred_files: []}]);

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
			<h4 class="py-0 my-0 font-normal">Device name</h4>
			<h3 class="py-0 my-0">Rtin</h3>
		</div>
		<div>
			<div class="hover:bg-gray-200 cursor-pointer p-2 rounded-lg">
				<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"><path d="M480-160q-33 0-56.5-23.5T400-240q0-33 23.5-56.5T480-320q33 0 56.5 23.5T560-240q0 33-23.5 56.5T480-160Zm0-240q-33 0-56.5-23.5T400-480q0-33 23.5-56.5T480-560q33 0 56.5 23.5T560-480q0 33-23.5 56.5T480-400Zm0-240q-33 0-56.5-23.5T400-720q0-33 23.5-56.5T480-800q33 0 56.5 23.5T560-720q0 33-23.5 56.5T480-640Z"/></svg>
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
			<p class="mt-4 mb-1">Currently</p>
			<h4 class="mt-0">Receiving from everyone</h4>
			<p>Everyone can share with you (you still need to approve each transfer).</p>
		</div>
		<div class="flex-1 flex flex-col h-full rounded-tl-[3rem] bg-white p-6" :class="{'items-center': requests.filter((el) => el.state >= 7).length === 0}">
			<h3 class="my-4">Ready to receive</h3>

			<div v-if="requests.filter((el) => el.state >= 7).length === 0" class="my-auto status-indicator status-indicator--success status-indicator--xl">
				<div class="circle circle--animated circle-main" />
				<div class="circle circle--animated circle-secondary" />
				<div class="circle circle--animated circle-tertiary" />
			</div>

			<div v-for="request in requests.filter((el) => el.state >= 7)" :key="request.name" class="bg-green-200 rounded-3xl flex flex-row gap-6 p-6">
				<div class="h-16 w-16 rounded-full bg-green-50"></div>
				<div class="flex-1 flex flex-col">
					<h4 class="mt-0">Martin's Phone</h4>
					<div v-if="request.state === 7" class="flex flex-col">
						<p class="mt-2 mb-0">Wants to share some file(s)</p>
						<div class="flex flex-row justify-end gap-4 mt-1">
							<p class="my-0 cursor-pointer p-2 px-3 hover:bg-green-100 rounded-lg font-semibold">Accept</p>
							<p class="my-0 cursor-pointer p-2 px-3 hover:bg-green-100 rounded-lg font-semibold">Decline</p>
						</div>
					</div>
					<div v-else-if="request.state === 8">
						<p class="mt-2 font-medium">Receiving...</p>
						<p>{{ request.transferred_files }}</p>
						<p></p>
					</div>
				</div>
			</div>
		</div>
	</div>
  </div>
</template>