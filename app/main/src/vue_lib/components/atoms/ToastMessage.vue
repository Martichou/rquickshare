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
		:class="{
			'bg-red-100 dark:bg-red-900': type === ToastType.Error,
			'bg-green-100 dark:bg-green-900': type === ToastType.Success,
			'bg-blue-100 dark:bg-blue-900': type === ToastType.Info
		}">
		<span class="text-sm text-gray-800 dark:text-gray-100">
			{{ message }}
		</span>
	</div>
</template>