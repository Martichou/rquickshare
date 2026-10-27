import { devtools } from '@vue/devtools'
import { createApp } from 'vue'
import { createPinia } from 'pinia'

import './vue_lib/assets/main.postcss'

import { initSystemTheme } from './theme'
import { installDevMock } from './dev-mock'

import App from './App.vue'

if (process.env.NODE_ENV === 'development') {
	devtools.connect('http://localhost', 8098)
}

if (import.meta.env.DEV && !('__TAURI_INTERNALS__' in window)) {
	installDevMock()
}

const pinia = createPinia();
const app = createApp(App)

app.use(pinia);

app.mount('#app')

initSystemTheme()
