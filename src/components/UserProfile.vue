<template>
    <div class="user-profile">
      <!-- 已登录状态 -->
      <div v-if="user" class="user-logged-in" @click="$emit('open-menu')">
        <div class="user-avatar" v-if="user.avatar">
          {{ user.avatar }}
        </div>
        <div class="user-avatar" v-else>
          {{ user.username.charAt(0).toUpperCase() }}
        </div>
        <span class="username">{{ user.username }}</span>
      </div>
      
      <!-- 未登录状态 -->
      <div v-else class="user-logged-out" @click="$emit('login')">
        <div class="user-icon">
          <UserIcon />
        </div>
        <span class="login-text">登录</span>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import UserIcon from '../assets/icons/UserIcon.vue';
  
  interface User {
    id: string;
    username: string;
    email: string;
    avatar?: string;
  }
  
  defineProps<{
    user: User | null;
  }>();
  
  defineEmits<{
    (e: 'login'): void;
    (e: 'logout'): void;
    (e: 'open-menu'): void;
  }>();
  </script>
  
  <style lang="scss" scoped>
  .user-profile {
    display: flex;
    align-items: center;
    cursor: pointer;
  }
  
  .user-logged-in, .user-logged-out {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 8px;
    border-radius: 4px;
    
    &:hover {
      background-color: rgba(255, 255, 255, 0.1);
    }
  }
  
  .user-avatar {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background-color: var(--avatar-bg);
    color: var(--avatar-text);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
  }
  
  .username, .login-text {
    font-size: 14px;
    color: var(--tab-active-text);
  }
  
  .user-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--tab-text);
  }
  </style>