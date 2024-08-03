import { devtools } from '@vue/devtools'
import { createApp } from 'vue'
import App from './App.vue'

import 'vue_lib/assets/main.postcss'
import { createPinia } from 'pinia'

if (process.env.NODE_ENV === 'development') {
	devtools.connect('http://localhost', 8098)
}

const pinia = createPinia();
const app = createApp(App)

app.use(pinia);

app.mount('#app')
