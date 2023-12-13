import { createApp } from 'vue';
import './styles.css';
import App from './App.vue';
import '@unocss/reset/tailwind.css';
import 'virtual:uno.css';
import 'ant-design-vue/dist/reset.css';

const app = createApp(App);

app.mount('#app');
