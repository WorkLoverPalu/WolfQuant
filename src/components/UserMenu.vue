<template>
    <div class="user-menu-overlay" @click="$emit('close')">
      <div class="user-menu" @click.stop>
        <div class="user-info">
          <div class="user-avatar">{{ userInitial }}</div>
          <div class="user-details">
            <div class="username">{{ user.username }}</div>
            <div class="email">{{ user.email }}</div>
          </div>
        </div>
        
        <div class="menu-divider"></div>
        
        <div class="menu-items">
          <button class="menu-item" @click="$emit('close')">
            <SettingsIcon />
            <span>设置</span>
          </button>
          
          <button class="menu-item" @click="$emit('close')">
            <RefreshIcon />
            <span>重新启动以更新</span>
          </button>
        </div>
        
        <div class="menu-divider"></div>
        
        <div class="theme-selector">
          <span class="theme-label">主题</span>
          <div class="theme-options">
            <button class="theme-option" :class="{ active: theme === 'system' }">
              <div class="theme-preview system-theme"></div>
              <span>系统</span>
            </button>
            <button class="theme-option" :class="{ active: theme === 'dark' }">
              <div class="theme-preview dark-theme"></div>
              <span>暗色</span>
            </button>
            <button class="theme-option" :class="{ active: theme === 'light' }">
              <div class="theme-preview light-theme"></div>
              <span>亮色</span>
            </button>
          </div>
        </div>
        
        <div class="menu-divider"></div>
        
        <button class="menu-item logout" @click="$emit('logout')">
          <LogoutIcon />
          <span>退出登录</span>
        </button>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { computed, ref } from 'vue';
  import SettingsIcon from '../assets/icons/SettingsIcon.vue';
  import RefreshIcon from '../assets/icons/RefreshIcon.vue';
  import LogoutIcon from '../assets/icons/LogoutIcon.vue';
  
  interface User {
    id: string;
    username: string;
    email: string;
    avatar?: string;
  }
  
  const props = defineProps<{
    user: User;
  }>();
  
  defineEmits<{
    (e: 'close'): void;
    (e: 'logout'): void;
  }>();
  
  // 当前主题，默认为暗色
  const theme = ref('dark');
  
  // 用户头像显示的首字母
  const userInitial = computed(() => {
    return props.user.avatar || props.user.username.charAt(0).toUpperCase();
  });
  </script>
  
  <style lang="scss" scoped>
  .user-menu-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 1000;
  }
  
  .user-menu {
    position: absolute;
    top: 40px; // 头部导航栏高度
    right: 16px;
    width: 280px;
    background-color: var(--modal-bg);
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    padding: 12px 0;
    z-index: 1001;
  }
  
  .user-info {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    
    .user-avatar {
      width: 40px;
      height: 40px;
      border-radius: 50%;
      background-color: var(--avatar-bg);
      color: var(--avatar-text);
      display: flex;
      align-items: center;
      justify-content: center;
      font-weight: bold;
      font-size: 18px;
      margin-right: 12px;
    }
    
    .user-details {
      flex: 1;
      
      .username {
        font-weight: 500;
        font-size: 14px;
        margin-bottom: 4px;
      }
      
      .email {
        font-size: 12px;
        color: var(--tab-text);
      }
    }
  }
  
  .menu-divider {
    height: 1px;
    background-color: var(--border-color);
    margin: 8px 0;
  }
  
  .menu-items {
    padding: 0 8px;
  }
  
  .menu-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 8px 12px;
    border-radius: 4px;
    background: transparent;
    border: none;
    color: var(--tab-active-text);
    cursor: pointer;
    text-align: left;
    font-size: 14px;
    
    svg {
      margin-right: 12px;
      color: var(--tab-text);
    }
    
    &:hover {
      background-color: rgba(255, 255, 255, 0.1);
    }
    
    &.logout {
      color: #ef4444;
      
      svg {
        color: #ef4444;
      }
    }
  }
  
  .theme-selector {
    padding: 8px 16px;
    
    .theme-label {
      display: block;
      font-size: 12px;
      color: var(--tab-text);
      margin-bottom: 8px;
    }
    
    .theme-options {
      display: flex;
      justify-content: space-between;
      gap: 8px;
    }
    
    .theme-option {
      flex: 1;
      display: flex;
      flex-direction: column;
      align-items: center;
      background: transparent;
      border: none;
      cursor: pointer;
      padding: 8px 4px;
      border-radius: 4px;
      
      &:hover {
        background-color: rgba(255, 255, 255, 0.05);
      }
      
      &.active {
        background-color: rgba(255, 255, 255, 0.1);
      }
      
      .theme-preview {
        width: 64px;
        height: 40px;
        border-radius: 4px;
        margin-bottom: 8px;
        border: 1px solid var(--border-color);
        overflow: hidden;
        
        &.system-theme {
          background: linear-gradient(to right, #1a1a1a 50%, #f5f5f5 50%);
        }
        
        &.dark-theme {
          background-color: #1a1a1a;
        }
        
        &.light-theme {
          background-color: #f5f5f5;
        }
      }
      
      span {
        font-size: 12px;
        color: var(--tab-text);
      }
    }
  }
  </style>