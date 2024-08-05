import { App } from 'vue';

import components from'./components';

export * from './types';

import utils from './utils';
export type UtilsType = typeof utils;

export const VueLib = {
	install: (app: App) => {
		app.component("ToastMessage", components.ToastMessage);
		app.component("ToastNotification", components.ToastNotification);

		app.provide('utils', utils);
	}
}

export { useToastStore } from './stores/useToastStore';

export function opt<T>(v?: T) {
	return v ?? null;
}