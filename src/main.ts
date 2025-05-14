import { createApp } from 'vue';
import App from './App.vue';
import EmptyTab from './components/EmptyTab.vue';
import { themeService } from './services/theme-service';
import { createPinia } from 'pinia'

// 初始化主题
themeService.applyTheme(themeService.getThemePreference());

const app = createApp(App);
const pinia = createPinia()

// 全局注册组件用于动态组件加载
app.component('EmptyTab', EmptyTab);


app.use(pinia)
app.mount('#app');