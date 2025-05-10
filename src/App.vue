<template>
  <div class="app-container">
    <header class="app-header">
      <div class="logo">WolfQuant</div>
      <div class="user-section">
        <button v-if="!isLoggedIn" class="login-button" @click="showLoginModal = true">
          登录
        </button>
        <div v-else class="user-avatar" @click="showUserMenu = true">
          {{ userInitial }}
        </div>
      </div>
    </header>
    
    <main class="app-content">
      <!-- 应用内容 -->
      <div v-if="isLoggedIn" class="welcome-message">
        欢迎回来，{{ user?.username }}！
      </div>
      <div v-else class="welcome-message">
        请登录以使用完整功能
      </div>
    </main>
    
    <!-- 登录模态框 -->
    <LoginModal 
      v-if="showLoginModal" 
      @close="showLoginModal = false"
      @login="handleLogin"
      @forgot-password="showForgotPasswordModal = true; showLoginModal = false"
      @register="showRegisterModal = true; showLoginModal = false"
    />
    
    <!-- 注册模态框 -->
    <RegisterModal 
      v-if="showRegisterModal" 
      @close="showLoginModal = true; showRegisterModal = false"
      @register="handleRegister"
    />
    
    <!-- 忘记密码模态框 -->
    <ForgotPasswordModal 
      v-if="showForgotPasswordModal" 
      @close="showLoginModal = true; showForgotPasswordModal = false"
      @reset-password="handleForgotPassword"
    />
    
    <!-- 用户菜单 -->
    <UserMenu 
      v-if="showUserMenu && user" 
      :user="user"
      @close="showUserMenu = false"
      @logout="handleLogout"
      @open-settings="handleOpenSettings"
      @open-profile="handleOpenProfile"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import LoginModal from './components/LoginModal.vue';
import RegisterModal from './components/RegisterModal.vue';
import ForgotPasswordModal from './components/ForgotPasswordModal.vue';
import UserMenu from './components/UserMenu.vue';
import { login, register, forgotPassword, logout, verifySession, getCurrentUser, isLoggedIn as checkIsLoggedIn, type User } from './services/auth-service';

// 状态
const showLoginModal = ref(false);
const showRegisterModal = ref(false);
const showForgotPasswordModal = ref(false);
const showUserMenu = ref(false);
const user = ref<User | null>(null);
const isLoggedIn = ref(false);

// 计算属性
const userInitial = computed(() => {
  return user.value?.username.charAt(0).toUpperCase() || '';
});

// 生命周期钩子
onMounted(async () => {
  // 验证会话
  await checkSession();
});

// 方法
async function checkSession() {
  isLoggedIn.value = checkIsLoggedIn();
  
  if (isLoggedIn.value) {
    user.value = getCurrentUser();
    
    // 验证会话有效性
    try {
      const verifiedUser = await verifySession();
      if (verifiedUser) {
        user.value = verifiedUser;
      } else {
        isLoggedIn.value = false;
        user.value = null;
      }
    } catch (error) {
      console.error('Session verification failed:', error);
      isLoggedIn.value = false;
      user.value = null;
    }
  }
}

async function handleLogin(username: string, password: string) {
  try {
    const response = await login(username, password);
    user.value = response.user;
    isLoggedIn.value = true;
    showLoginModal.value = false;
    
    // 显示成功消息
    alert(response.message);
  } catch (error: any) {
    alert(error.error || '登录失败');
  }
}

async function handleRegister(email: string, password: string, username: string) {
  try {
    const response = await register(email, password, username);
    showRegisterModal.value = false;
    showLoginModal.value = true;
    
    // 显示成功消息
    alert(response.message);
  } catch (error: any) {
    alert(error.error || '注册失败');
  }
}

async function handleForgotPassword(email: string) {
  try {
    const response = await forgotPassword(email);
    showForgotPasswordModal.value = false;
    
    // 显示成功消息
    alert(response.message);
  } catch (error: any) {
    alert(error.error || '发送重置链接失败');
  }
}

async function handleLogout() {
  try {
    await logout();
    user.value = null;
    isLoggedIn.value = false;
    showUserMenu.value = false;
    
    // 显示成功消息
    alert('退出登录成功');
  } catch (error: any) {
    alert(error.error || '退出登录失败');
  }
}

function handleOpenSettings() {
  showUserMenu.value = false;
  // 打开设置页面的逻辑
  alert('打开设置页面');
}

function handleOpenProfile() {
  showUserMenu.value = false;
  // 打开个人资料页面的逻辑
  alert('打开个人资料页面');
}
</script>

<style lang="scss" scoped>
.app-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 16px;
  height: 48px;
  background-color: var(--header-bg);
  border-bottom: 1px solid var(--border-color);
  
  .logo {
    font-weight: 600;
    font-size: 18px;
  }
  
  .user-section {
    display: flex;
    align-items: center;
    
    .login-button {
      padding: 6px 12px;
      background-color: var(--button-primary);
      color: white;
      border: none;
      border-radius: 4px;
      font-size: 14px;
      cursor: pointer;
      
      &:hover {
        background-color: var(--button-primary-hover);
      }
    }
    
    .user-avatar {
      width: 32px;
      height: 32px;
      border-radius: 50%;
      background-color: var(--avatar-bg);
      color: var(--avatar-text);
      display: flex;
      align-items: center;
      justify-content: center;
      font-weight: bold;
      cursor: pointer;
    }
  }
}

.app-content {
  flex: 1;
  padding: 24px;
  
  .welcome-message {
    font-size: 18px;
    text-align: center;
    margin-top: 48px;
  }
}
</style>