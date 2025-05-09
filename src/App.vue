<template>
  <ThemeProvider>
    <div class="app-container">
      <header class="app-header">
        <div class="tabs-container">
          <TabItem 
            v-for="(tab, index) in tabs" 
            :key="index" 
            :tab="tab" 
            :active="activeTabIndex === index"
            @click="switchTab(index)"
            @close="closeTab(index)"
          />
          <button class="new-tab-button" @click="addNewTab">
            <PlusIcon />
          </button>
        </div>
        
        <!-- 用户登录组件 -->
        <HeaderUserProfile 
          :user="currentUser" 
          @login="openLoginModal" 
          @logout="handleLogout"
          @open-menu="isUserMenuOpen = true"
          @open-profile="openUserProfileTab"
        />
      </header>
      
      <main class="app-content">
        <component 
          :is="activeTabComponent" 
          v-bind="activeTabProps"
        ></component>
      </main>
  
      <!-- 登录模态框 -->
      <LoginModal 
        v-if="showLoginModal" 
        @close="showLoginModal = false"
        @login="handleLogin"
        @forgot-password="openForgotPasswordModal"
        @register="openRegisterModal"
      />
  
      <!-- 注册模态框 -->
      <RegisterModal 
        v-if="showRegisterModal" 
        @close="showRegisterModal = false"
        @register="handleRegister"
      />
  
      <!-- 忘记密码模态框 -->
      <ForgotPasswordModal 
        v-if="showForgotPasswordModal" 
        @close="showForgotPasswordModal = false"
        @reset-password="handleResetPassword"
      />
  
      <!-- 用户菜单 -->
      <UserMenu 
        v-if="isUserMenuOpen && currentUser" 
        :user="currentUser"
        @close="isUserMenuOpen = false"
        @logout="handleLogout"
        @open-settings="openSettingsModal"
        @open-profile="openUserProfileTab"
      />
      
      <!-- 设置弹窗 -->
      <SettingsModal 
        v-if="showSettingsModal"
        @close="showSettingsModal = false"
        @save="handleSaveSettings"
      />
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
  import './styles/transitions.scss';
  
  // 用户数据
  const currentUser = ref({
    username: 'flashdanacom',
    email: 'user@example.com',
    avatar: 'F'
  });
  
  // 模态框状态
  const showLoginModal = ref(false);
  const showRegisterModal = ref(false);
  const showForgotPasswordModal = ref(false);
  const showSettingsModal = ref(false);
  const isUserMenuOpen = ref(false);
  
  // 打开登录模态框
  const openLoginModal = () => {
    showLoginModal.value = true;
  };
  
  // 处理登录
  const handleLogin = (userData: any) => {
    currentUser.value = userData;
    showLoginModal.value = false;
  };
  
  // 处理登出
  const handleLogout = () => {
    currentUser.value = null;
    isUserMenuOpen.value = false;
  };
  
  // 打开注册模态框
  const openRegisterModal = () => {
    showLoginModal.value = false;
    showRegisterModal.value = true;
  };
  
  // 处理注册
  const handleRegister = (userData: any) => {
    currentUser.value = userData;
    showRegisterModal.value = false;
  };
  
  // 打开忘记密码模态框
  const openForgotPasswordModal = () => {
    showLoginModal.value = false;
    showForgotPasswordModal.value = true;
  };
  
  // 处理重置密码
  const handleResetPassword = (email: string) => {
    console.log('重置密码邮件已发送到:', email);
    showForgotPasswordModal.value = false;
  };
  
  // 打开设置模态框
  const openSettingsModal = () => {
    isUserMenuOpen.value = false;
    showSettingsModal.value = true;
  };
  
  // 处理保存设置
  const handleSaveSettings = (settings: any) => {
    console.log('保存设置:', settings);
    showSettingsModal.value = false;
  };
  
  // 标签相关代码
  const tabs = ref([
    { title: 'Tab 1', component: shallowRef({ template: '<div>默认标签内容</div>' }), props: {} }
  ]);
  const activeTabIndex = ref(0);
  
  // 获取当前激活的标签组件
  const activeTabComponent = computed(() => {
    if (tabs.value.length === 0) {
      return { template: '<div>无内容</div>' };
    }
    return tabs.value[activeTabIndex.value].component;
  });
  
  // 获取当前激活的标签属性
  const activeTabProps = computed(() => {
    if (tabs.value.length === 0) return {};
    return tabs.value[activeTabIndex.value].props;
  });
  
  // 切换标签
  const switchTab = (index: number) => {
    activeTabIndex.value = index;
  };
  
  // 关闭标签
  const closeTab = (index: number) => {
    tabs.value.splice(index, 1);
    if (tabs.value.length === 0) {
      addNewTab();
    } else if (activeTabIndex.value >= tabs.value.length) {
      activeTabIndex.value = tabs.value.length - 1;
    }
  };
  
  // 添加新标签
  const addNewTab = () => {
    tabs.value.push({ 
      title: `Tab ${tabs.value.length + 1}`, 
      component: shallowRef({ template: '<div>新标签内容</div>' }), 
      props: {} 
    });
    activeTabIndex.value = tabs.value.length - 1;
  };
  
  // 打开用户个人中心标签
  const openUserProfileTab = () => {
    // 检查是否已经存在用户个人中心标签
    const existingTabIndex = tabs.value.findIndex(tab => tab.title === '个人中心');
    
    if (existingTabIndex !== -1) {
      // 如果已存在，切换到该标签
      activeTabIndex.value = existingTabIndex;
    } else {
      // 如果不存在，创建新标签
      tabs.value.push({
        title: '个人中心',
        component: markRaw(UserProfile), // 使用markRaw避免Vue的响应式系统对组件的代理
        props: {
          userData: currentUser.value
        }
      });
      activeTabIndex.value = tabs.value.length - 1;
    }
    
    // 关闭用户菜单
    isUserMenuOpen.value = false;
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
    --header-hover-bg: rgba(255, 255, 255, 0.05);
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
    overflow: hidden;
  }
  
  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 16px;
    height: 60px;
    background-color: var(--header-bg);
    border-bottom: 1px solid var(--border-color);
  }
  
  .tabs-container {
    display: flex;
    align-items: center;
    overflow-x: auto;
    flex: 1;
    margin-right: 16px;
    
    &::-webkit-scrollbar {
      height: 4px;
    }
    
    &::-webkit-scrollbar-track {
      background: transparent;
    }
    
    &::-webkit-scrollbar-thumb {
      background: var(--border-color);
      border-radius: 2px;
    }
  }
  
  .new-tab-button {
    background: transparent;
    border: none;
    color: var(--tab-text);
    padding: 8px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    
    &:hover {
      color: var(--tab-active-text);
    }
  }
  
  .app-content {
    flex: 1;
    overflow: auto;
    background-color: var(--bg-color);
  }
  </style>