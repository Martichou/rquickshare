<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onMounted, onUnmounted, ref } from 'vue';

const win = getCurrentWindow();
const maximized = ref(false);
let unlisten: (() => void) | undefined;

async function sync() {
	maximized.value = await win.isMaximized().catch(() => false);
	document.documentElement.classList.toggle('maximized', maximized.value);
}

onMounted(async () => {
	await sync();
	unlisten = await win.onResized(sync).catch(() => undefined);
});
onUnmounted(() => unlisten?.());
</script>

<template>
	<div data-tauri-drag-region class="flex flex-row justify-end items-center h-10 px-2 select-none shrink-0" @dblclick="win.toggleMaximize()">
		<div class="flex flex-row items-center gap-1">
			<div class="titlebar-btn" title="Minimize" @click="win.minimize()">
				<svg xmlns="http://www.w3.org/2000/svg" height="16" width="16" viewBox="0 0 16 16"><path d="M3 8h10v1.2H3z" /></svg>
			</div>
			<div class="titlebar-btn" :title="maximized ? 'Restore' : 'Maximize'" @click="win.toggleMaximize()">
				<svg v-if="!maximized" xmlns="http://www.w3.org/2000/svg" height="16" width="16" viewBox="0 0 16 16"><path d="M3.5 3.5h9v9h-9zm1.2 1.2v6.6h6.6V4.7z" /></svg>
				<svg v-else xmlns="http://www.w3.org/2000/svg" height="16" width="16" viewBox="0 0 16 16"><path d="M5.5 3h7.500v7.500h-1.200V4.200H5.500zM3 5.500h7.500V13H3zm1.200 1.200v5.100h5.100V6.700z" /></svg>
			</div>
			<div class="titlebar-btn hover:!bg-red-500 hover:!text-white" title="Close" @click="win.close()">
				<svg xmlns="http://www.w3.org/2000/svg" height="16" width="16" viewBox="0 0 16 16"><path d="m4 3.150 4 4 4-4 .850.850-4 4 4 4-.850.850-4-4-4 4L3.150 12l4-4-4-4z" /></svg>
			</div>
		</div>
	</div>
</template>
