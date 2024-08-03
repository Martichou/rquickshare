import { defineStore } from "pinia";
import { ref } from "vue";

export interface Toast {
    id: number;
    type: ToastType;
    message: string;
}

export enum ToastType {
    Success = "SUCCESS",
    Error = "ERROR",
    Info = "INFO",
}

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