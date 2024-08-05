<script lang="ts" setup>
import { PropType } from 'vue';
import { ToastType } from '../../types';

defineProps({
	id: {
		type: Number,
		required: true,
	},
	type: {
		type: String as PropType<ToastType>,
		default: ToastType.Success,
	},
	message: {
		type: String,
		required: true,
	},
});
</script>

<template>
	<div
		class="w-full p-4 rounded-xl shadow-md"
		:class="{'bg-red-100': type === ToastType.Error, 'bg-green-100': type === ToastType.Success, 'bg-blue-100': type === ToastType.Info}">
		<span class="text-sm text-gray-800">
			{{ message }}
		</span>
	</div>
</template>