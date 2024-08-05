import { devtools } from '@vue/devtools'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { VueLib } from 'vue_lib';

import 'vue_lib/assets/main.postcss'

import App from './App.vue'

if (process.env.NODE_ENV === 'development') {
	devtools.connect('http://localhost', 8098)
}

const pinia = createPinia();
const app = createApp(App)

app.use(pinia);

// Not sure why as any is needed here
app.use(VueLib as any);

app.mount('#app')
