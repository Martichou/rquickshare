<script setup lang="ts">
import { computed } from 'vue';
import { useToastStore } from '../../stores/useToastStore';
import ToastMessage from '../atoms/ToastMessage.vue';

const store = useToastStore();
const toasts = computed(() => store.toasts);

function removeToast(id: number) {
	store.removeToast(id);
}
</script>

<template>
	<div class="fixed top-4 right-4 flex flex-col gap-4 z-50">
		<div
			v-for="toast in toasts"
			:key="toast.id">
			<ToastMessage
				:id="toast.id"
				:message="toast.message"
				:type="toast.type"
				@remove="removeToast" />
		</div>
	</div>
</template>