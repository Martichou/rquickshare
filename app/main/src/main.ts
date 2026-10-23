import { devtools } from '@vue/devtools'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import {i18n} from './i18n'

import './vue_lib/assets/main.postcss'

import App from './App.vue'

if (process.env.NODE_ENV === 'development') {
	devtools.connect('http://localhost', 8098)
}

const pinia = createPinia();
const app = createApp(App)

app.use(pinia);
app.use(i18n);

app.mount('#app')
