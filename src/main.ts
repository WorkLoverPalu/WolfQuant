import { createApp } from 'vue';
import App from './App.vue';
import EmptyTab from './components/EmptyTab.vue';
import TradingView from './components/TradingView.vue';

const app = createApp(App);

// 全局注册组件用于动态组件加载
app.component('EmptyTab', EmptyTab);
app.component('TradingView', TradingView);

app.mount('#app');