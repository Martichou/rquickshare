import { defineStore } from "pinia";
import { ref } from "vue";
import { Toast, ToastType } from "../types";

export const useToastStore = defineStore("toasts", () => {
	const toasts = ref<Toast[]>([]);

	function addToast(message: string, type: ToastType) {
		const toast = {
			id: Date.now(),
			message,
			type,
		};
		toasts.value.push(toast);
		setTimeout(() => removeToast(toast.id), 3000);
	}

	function removeToast(id: number) {
		toasts.value = toasts.value.filter((toast) => toast.id !== id);
	}

	return { toasts, addToast, removeToast };
});