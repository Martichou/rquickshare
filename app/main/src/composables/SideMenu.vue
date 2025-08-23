<script setup lang="ts">
import { TauriVM } from '../vue_lib/helper/ParamsHelper';
import { PropType } from 'vue';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emits = defineEmits(['invertVisibility', 'clearSending']);
</script>

<template>
	<div class="w-72 p-3" v-if="props.vm.outboundPayload === undefined">
		<p class="mt-4 mb-2 pt-3 px-3">
			{{ $t('labels.visibilityState') }}
		</p>
		<h4 tabindex="0" role="button" class="btn font-medium flex flex-row !justify-between w-full
            items-center rounded-xl active:scale-95 transition duration-150 ease-in-out p-3"
			@click="emits('invertVisibility')">
			<span v-if="props.vm.visibility === 'Visible'">{{ $t('labels.alwaysVisible') }}</span>
			<span v-else-if="props.vm.visibility === 'Invisible'">{{ $t('labels.alwaysHidden') }}</span>
			<span v-else>{{ $t('labels.tempVisible') }}</span>

			<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
				:class="{ 'rotate-180': props.vm.visibility === 'Invisible' }">
				<path d="M504-480 320-664l56-56 240 240-240 240-56-56 184-184Z" />
			</svg>
		</h4>
		<p class="text-xs mt-2 pb-3 px-3">
			<span v-if="props.vm.visibility === 'Visible'">
				{{ $t('info.alwaysVisibleDetails') }}
			</span>
			<span v-else-if="props.vm.visibility === 'Invisible'">
				{{ $t('info.alwaysHiddenDetail0') }}
				<br>
				<br>
				{{ $t('info.alwaysHiddenDetail1') }}
			</span>
			<span v-else>
				{{ $t('info.tempVisibleDetail') }}
			</span>
		</p>
	</div>
	<div class="w-72 p-6 flex flex-col justify-between" v-else>
		<div>
			<p class="mt-4 mb-2">
				{{ $t('info.fileInSharing', { n: props.vm.outboundPayload.Files.length },
					props.vm.outboundPayload.Files.length) }}
			</p>
			<div class="bg-white w-32 h-32 rounded-2xl mb-2 flex justify-center items-center">
				<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" class="w-8 h-8">
					<!-- eslint-disable-next-line -->
					<path
						d="M240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v480q0 33-23.5 56.5T720-80H240Zm280-520v-200H240v640h480v-440H520ZM240-800v200-200 640-640Z" />
				</svg>
			</div>
			<p v-for="f in props.vm.outboundPayload.Files" :key="f"
				class="overflow-hidden whitespace-nowrap text-ellipsis">
				{{ f.split('/').pop() }}
			</p>

			<p class="text-xs mt-3">
				{{ $t('info.connectivityWarning') }}
			</p>
		</div>

		<p @click="emits('clearSending')"
			class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out w-fit">
			{{ $t('buttons.cancel') }}
		</p>
	</div>
</template>