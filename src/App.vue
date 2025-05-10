<template>
  <ThemeProvider>
    <div class="app-container">
      <header class="app-header">
        <div class="tabs-container">
          <TabItem v-for="(tab, index) in tabs" :key="index" :tab="tab" :active="activeTabIndex === index"
            @click="switchTab(index)" @close="closeTab(index)" />
          <button class="new-tab-button" @click="addNewTab">
            <PlusIcon />
          </button>
        </div>

        <!-- 用户登录组件 -->
        <HeaderUserProfile :user="currentUser" @login="openLoginModal" @logout="handleLogout"
          @open-menu="isUserMenuOpen = true" @open-profile="openUserProfileTab" />
      </header>
      <main class="app-content">
        <component :is="activeTabComponent" v-bind="activeTabProps"></component>
      </main>

      <!-- 底部行情组件 -->
      <MarketFooter :show-account="true" :show-nav="true" :market-data="currentMarketData" :current-user="currentUser"
        @nav-click="handleFooterNavClick" @market-click="handleMarketClick" @open-tab="handleOpenTab" />

      <!-- 登录模态框 -->
      <LoginModal v-if="showLoginModal" @close="showLoginModal = false" @login="handleLogin"
        @forgot-password="openForgotPasswordModal" @register="openRegisterModal" />

      <!-- 注册模态框 -->
      <RegisterModal v-if="showRegisterModal" @close="showRegisterModal = false" @register="handleRegister" />

      <!-- 忘记密码模态框 -->
      <ForgotPasswordModal v-if="showForgotPasswordModal" @close="showForgotPasswordModal = false"
        @reset-password="handleResetPassword" />

      <!-- 用户菜单 -->
      <UserMenu v-if="isUserMenuOpen && currentUser" :user="currentUser" @close="isUserMenuOpen = false"
        @logout="handleLogout" @open-settings="openSettingsModal" @open-profile="openUserProfileTab" />

      <!-- 设置弹窗 -->
      <SettingsModal v-if="showSettingsModal" @close="showSettingsModal = false" @save="handleSaveSettings" />
    </div>
  </ThemeProvider>
</template>

<script setup lang="ts">
import { ref, computed, markRaw, shallowRef } from 'vue';
import ThemeProvider from './components/ThemeProvider.vue';
import TabItem from './components/tabs/TabItem.vue';
import HeaderUserProfile from './components/header/HeaderUserProfile.vue';
import LoginModal from './components/header/LoginModal.vue';
import RegisterModal from './components/header/RegisterModal.vue';
import ForgotPasswordModal from './components/header/ForgotPasswordModal.vue';
import UserMenu from './components/header/UserMenu.vue';
import SettingsModal from './components/header/SettingsModal.vue';
import PlusIcon from './assets/icons/PlusIcon.vue';
import UserProfile from './components/UserProfile.vue';
import MarketWatchlist from './components/MarketWatchlist.vue';
import MarketFooter from './components/footer/MarketFooter.vue';
import './styles/transitions.scss';

interface Tab {
  id: string;
  title: string;
  component: any;
  props?: Record<string, any>;
  closable: boolean;
}

interface User {
  id: string;
  username: string;
  email: string;
  avatar?: string;
}

// 市场数据类型
interface MarketData {
  name: string;
  symbol: string;
  price: string;
  change: string;
  percentChange: string;
  high: string;
  low: string;
  open: string;
  prevClose: string;
  volume: string;
  updateTime: string;
}

// 用户状态
const currentUser = ref<User | null>(null);
const showLoginModal = ref(false);
const showRegisterModal = ref(false);
const showForgotPasswordModal = ref(false);
const isUserMenuOpen = ref(false);
const showSettingsModal = ref(false);

// 标签页状态
const tabs = ref<Tab[]>([
  {
    id: 'market-watchlist',
    title: 'MarketWatchlist',
    component: markRaw(MarketWatchlist),
    props: {},
    closable: true
  },
  {
    id: '2',
    title: 'BIOUSDT',
    component: 'WolfQuant',
    props: { symbol: 'BIOUSDT', price: '0.08221', change: '+16.1%' },
    closable: true
  },
  {
    id: '3',
    title: 'New Tab',
    component: 'EmptyTab',
    closable: false
  }
]);

const activeTabIndex = ref(0);

// 计算属性用于动态加载组件
const activeTabComponent = computed(() => {
  const componentName = tabs.value[activeTabIndex.value]?.component || 'EmptyTab';
  return componentName;
});

const activeTabProps = computed(() => {
  return tabs.value[activeTabIndex.value]?.props || {};
});

// 当前市场数据
const currentMarketData = ref<MarketData>({
  name: '上证指数',
  symbol: 'sh000001',
  price: '3342.00',
  change: '-10.00',
  percentChange: '-0.30%',
  high: '3351.22',
  low: '3335.13',
  open: '3350.41',
  prevClose: '3352.00',
  volume: '4648.62亿',
  updateTime: '2025-05-09 15:30:39'
});

// 标签页操作
const switchTab = (index: number) => {
  activeTabIndex.value = index;
};

const closeTab = (index: number) => {
  if (tabs.value[index].closable) {
    tabs.value = tabs.value.filter((_, i) => i !== index);
    // 如果关闭的是当前活动标签页，切换到第一个标签页
    if (activeTabIndex.value === index) {
      activeTabIndex.value = 0;
    } else if (activeTabIndex.value > index) {
      // 如果关闭的标签页在当前活动标签页之前，调整索引
      activeTabIndex.value--;
    }
  }
};

const addNewTab = () => {
  const newTabId = `tab-${Date.now()}`;
  tabs.value.push({
    id: newTabId,
    title: 'New Tab',
    component: 'EmptyTab',
    closable: true
  });
  activeTabIndex.value = tabs.value.length - 1;
};

// 用户相关操作
const openLoginModal = () => {
  showLoginModal.value = true;
  isUserMenuOpen.value = false;
};

const openRegisterModal = () => {
  showLoginModal.value = false;
  showRegisterModal.value = true;
};

const openForgotPasswordModal = () => {
  showLoginModal.value = false;
  showForgotPasswordModal.value = true;
};

const openSettingsModal = () => {
  isUserMenuOpen.value = false;
  showSettingsModal.value = true;
};

const handleLogin = (username: string, password: string) => {
  // 这里应该是实际的登录逻辑，目前使用模拟数据
  currentUser.value = {
    id: '1',
    username: username,
    email: `${username}@example.com`,
    avatar: username.charAt(0).toUpperCase()
  };
  showLoginModal.value = false;
};

const handleRegister = (email: string, password: string, username: string) => {
  // 这里应该是实际的注册逻辑
  currentUser.value = {
    id: Date.now().toString(),
    username: username,
    email: email,
    avatar: username.charAt(0).toUpperCase()
  };
  showRegisterModal.value = false;
};

const handleResetPassword = (email: string) => {
  // 这里应该是实际的重置密码逻辑
  console.log(`重置密码链接已发送至: ${email}`);
  showForgotPasswordModal.value = false;
};

const handleLogout = () => {
  currentUser.value = null;
  isUserMenuOpen.value = false;
};

const handleSaveSettings = (settings: any) => {
  // 这里处理保存设置的逻辑
  console.log('保存设置:', settings);
  showSettingsModal.value = false;
};

// 打开用户个人中心标签
const openUserProfileTab = () => {
  console.log("open user")
  // 检查是否已经存在用户个人中心标签
  const existingTabIndex = tabs.value.findIndex(tab => tab.title === '个人中心');

  if (existingTabIndex !== -1) {
    // 如果已存在，切换到该标签
    activeTabIndex.value = existingTabIndex;
  } else {
    // 如果不存在，创建新标签
    const newTabId = `tab-${Date.now()}`;
    tabs.value.push({
      id: newTabId,
      title: '个人中心',
      component: markRaw(UserProfile), // 使用markRaw避免Vue的响应式系统对组件的代理
      closable: true,
      props: {
        userData: currentUser.value
      }
    });
    activeTabIndex.value = tabs.value.length - 1;
  }

  // 关闭用户菜单
  isUserMenuOpen.value = false;
};

// 底部行情组件相关操作
const handleFooterNavClick = (index: number, item: any) => {
  console.log(`底部导航点击: ${item.label || '未命名'} (索引: ${index})`);
};

const handleMarketClick = (market: MarketData) => {
  console.log(`市场点击: ${market.name} (${market.symbol})`);

  // 可以在这里打开市场详情标签页
  const existingTabIndex = tabs.value.findIndex(tab =>
    tab.props && tab.props.symbol === market.symbol
  );

  if (existingTabIndex !== -1) {
    // 如果已存在，切换到该标签
    activeTabIndex.value = existingTabIndex;
  } else {
    // 如果不存在，创建新标签
    const newTabId = `tab-${Date.now()}`;
    tabs.value.push({
      id: newTabId,
      title: market.name,
      component: 'MarketDetail', // 假设有一个市场详情组件
      closable: true,
      props: {
        symbol: market.symbol,
        name: market.name,
        price: market.price,
        change: market.change,
        percentChange: market.percentChange
      }
    });
    activeTabIndex.value = tabs.value.length - 1;
  }
};

// 处理底部组件打开新标签页
const handleOpenTab = (tabData: any) => {
  // 检查是否已经存在相同ID的标签
  const existingTabIndex = tabs.value.findIndex(tab => tab.id === tabData.id);

  if (existingTabIndex !== -1) {
    // 如果已存在，切换到该标签
    activeTabIndex.value = existingTabIndex;
  } else {
    // 如果不存在，创建新标签
    tabs.value.push({
      id: tabData.id || `tab-${Date.now()}`,
      title: tabData.title || 'New Tab',
      component: typeof tabData.component === 'string' ? tabData.component : markRaw(tabData.component),
      closable: tabData.closable !== undefined ? tabData.closable : true,
      props: tabData.props || {}
    });
    activeTabIndex.value = tabs.value.length - 1;
  }
};
</script>


<style lang="scss">
:root {
  --bg-color: #121212;
  --header-bg: #1a1a1a;
  --tab-bg: #252525;
  --tab-active-bg: #2a2a2a;
  --tab-text: #a0a0a0;
  --tab-active-text: #ffffff;
  --border-color: #333333;
  --avatar-bg: #4a4a4a;
  --avatar-text: #ffffff;
  --modal-bg: #1e1e1e;
  --input-bg: #2c2c2c;
  --button-primary: #2563eb;
  --button-primary-hover: #3b82f6;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  background-color: var(--bg-color);
  color: var(--tab-active-text);
  transition: background-color 0.3s ease, color 0.3s ease;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: var(--header-bg);
  border-bottom: 1px solid var(--border-color);
  padding: 0 16px;
  height: 40px;
  -webkit-app-region: drag;
  /* 整个头部可拖动 */
}

.tabs-container {
  display: flex;
  height: 100%;
  overflow-x: auto;
  scrollbar-width: none;
  /* Firefox */
  flex: 1;
  /* 让标签容器占据剩余空间 */
  -webkit-app-region: no-drag;
  /* 标签区域不可拖动 */

  &::-webkit-scrollbar {
    display: none;
    /* Chrome, Safari, Edge */
  }
}

.new-tab-button {
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--tab-text);
  height: 100%;
  padding: 0 8px;
  cursor: pointer;
  -webkit-app-region: no-drag;
  /* 按钮不可拖动 */

  &:hover {
    color: var(--tab-active-text);
  }
}

.user-profile {
  -webkit-app-region: no-drag;
  /* 用户资料区域不可拖动 */
}

.app-content {
  flex: 1;
  overflow: auto;
  background-color: var(--bg-color);
}


/* 添加底部行情组件的样式调整 */
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.app-content {
  flex: 1;
  overflow: auto;
  padding-bottom: 48px;
  /* 为底部组件留出空间 */
}
</style>