import { createApp } from 'vue';
import App from './App.vue';
import EmptyTab from './components/EmptyTab.vue';
import { createPinia } from 'pinia'
import { useThemeStore } from "./stores/themeStore"
import { useUserStore } from "./stores/userStore"

// 导入全局样式
import "./styles/common.css"
import "./styles/components.css"
import "./styles/transitions.scss"


const app = createApp(App);
const pinia = createPinia()
app.use(pinia)


// 全局注册组件用于动态组件加载
app.component('EmptyTab', EmptyTab);

// 初始化用户状态
const userStore = useUserStore()
// 初始化主题
const themeStore = useThemeStore()

userStore.initUser()
themeStore.initTheme()


app.mount('#app');