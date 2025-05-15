<template>
  <ThemeProvider>
    <div class="app-container">
      <header class="app-header">
        <div class="tabs-container">
          <TabItem v-for="(tab, index) in tabStore.tabs" :key="tab.id" :tab="tab"
            :active="tabStore.activeTabIndex === index" @click="tabStore.switchTab(index)"
            @close="tabStore.closeTab(index)" />
          <button class="new-tab-button" @click="tabStore.addNewTab()">
            <PlusIcon />
          </button>
        </div>

        <!-- header右侧用户登录状态组件 -->
        <HeaderUserProfile :user="userStore.user" @login="openLoginModal" @logout="userStore.logout"
          @open-menu="isUserMenuOpen = true" />
      </header>

      <main class="app-content">
        <component :is="tabStore.activeTabComponent" v-bind="tabStore.activeTabProps"></component>
      </main>

      <!-- 底部行情组件 -->
      <MarketFooter :show-account="true" :show-nav="true" :market-data="marketStore.currentMarketData"
        :current-user="userStore.user" @nav-click="handleFooterNavClick" @market-click="handleMarketClick" />

      <!-- 右侧导航组件 -->
      <SideNavigation :current-user="userStore.user" @open-tab="tabStore.openTab" />

      <!-- 登录模态框 -->
      <LoginModal v-if="showLoginModal" @close="showLoginModal = false" @login-success="handleLogin"
        @forgot-password="openForgotPasswordModal" @register="openRegisterModal" />

      <!-- 注册模态框 -->
      <RegisterModal v-if="showRegisterModal" @close="showRegisterModal = false" @login-success="handleLogin" />

      <!-- 忘记密码模态框 -->
      <ForgotPasswordModal v-if="showForgotPasswordModal" @close="showForgotPasswordModal = false" />

      <!-- 用户菜单 -->
      <UserMenu v-if="isUserMenuOpen && userStore.user"  @close="isUserMenuOpen = false"
        @logout="userStore.logout" @open-settings="openSettingsModal" @open-profile="openUserProfileTab"
        @new-tab="tabStore.addNewTab()" />

      <!-- 设置弹窗 -->
      <SettingsModal v-if="showSettingsModal" @close="showSettingsModal = false" @save="handleSaveSettings" />
    </div>
  </ThemeProvider>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import ThemeProvider from './components/ThemeProvider.vue';
import TabItem from './components/tabs/TabItem.vue';
import HeaderUserProfile from './components/header/HeaderUserProfile.vue';
import LoginModal from './components/header/LoginModal.vue';
import RegisterModal from './components/header/RegisterModal.vue';
import ForgotPasswordModal from './components/header/ForgotPasswordModal.vue';
import UserMenu from './components/header/UserMenu.vue';
import SettingsModal from './components/header/SettingsModal.vue';
import PlusIcon from './assets/icons/PlusIcon.vue';
import MarketFooter from './components/footer/MarketFooter.vue';
import SideNavigation from './components/right/SideNavigation.vue';

// 导入状态管理
import { useUserStore } from './stores/userStore';
import { useTabStore } from './stores/tabStore';
import { useAssetStore } from './stores/assetStore.ts';
import { useThemeStore } from './stores/themeStore';

// 使用状态管理
const userStore = useUserStore();
const tabStore = useTabStore();
const marketStore = useAssetStore();
const themeStore = useThemeStore();

// 模态框状态
const showLoginModal = ref(false);
const showRegisterModal = ref(false);
const showForgotPasswordModal = ref(false);
const isUserMenuOpen = ref(false);
const showSettingsModal = ref(false);

// 打开登录模态框
const openLoginModal = (pageId) => {
  showForgotPasswordModal.value = false;
  showRegisterModal.value = false;
  showLoginModal.value = true;
  isUserMenuOpen.value = false;
};

// 打开注册模态框
const openRegisterModal = () => {
  showLoginModal.value = false;
  showRegisterModal.value = true;
};

// 打开忘记密码模态框
const openForgotPasswordModal = () => {
  showLoginModal.value = false;
  showForgotPasswordModal.value = true;
};

// 打开设置模态框
const openSettingsModal = () => {
  isUserMenuOpen.value = false;
  showSettingsModal.value = true;
};

// 处理登录成功
const handleLogin = (user: any, password: any) => {
  showLoginModal.value = false;
  showRegisterModal.value = false;
};



// 处理保存设置
const handleSaveSettings = (settings: any) => {
  // 保存设置逻辑
  if (settings.theme) {
    themeStore.setTheme(settings.theme);
  }
  showSettingsModal.value = false;
};

// 打开用户个人中心标签
const openUserProfileTab = () => {
  tabStore.openUserProfileTab(userStore.user);
  isUserMenuOpen.value = false;
};

// 底部行情组件相关操作
const handleFooterNavClick = (index: any, item: any) => {
  console.log(`底部导航点击: ${item.label || '未命名'} (索引: ${index})`);
};

// 处理市场点击
const handleMarketClick = (market: any) => {
  tabStore.openTab({
    id: `market-${market.symbol}`,
    title: market.name,
    component: 'MarketDetail',
    props: {
      symbol: market.symbol,
      name: market.name,
      price: market.price,
      change: market.change,
      percentChange: market.percentChange
    },
    closable: true
  });
};
</script>
