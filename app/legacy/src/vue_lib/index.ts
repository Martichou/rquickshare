import ToastNotification from './components/organisms/ToastNotification.vue';
import ToastMessage from './components/atoms/ToastMessage.vue';

export * from './types';
export * from './utils';

export { ToastNotification, ToastMessage };

export { useToastStore } from './stores/useToastStore';

export function opt<T>(v?: T) {
	return v ?? null;
}