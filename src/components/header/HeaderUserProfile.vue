<template>
  <div class="user-profile-header">
    <div 
      v-if="user" 
      class="user-info" 
      @click="$emit('open-profile')"
    >
      <div class="avatar">{{ user.avatar || user.username.charAt(0).toUpperCase() }}</div>
      <div class="user-details">
        <div class="username">{{ user.username }}</div>
        <div class="email">{{ user.email }}</div>
      </div>
      <button class="menu-button" @click.stop="$emit('open-menu')">
        <i class="icon-chevron-down"></i>
      </button>
    </div>
    
    <button 
      v-else 
      class="login-button" 
      @click="$emit('login')"
    >
      登录
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps({
  user: {
    type: Object,
    default: null
  }
});

defineEmits(['login', 'logout', 'open-menu', 'open-profile']);
</script>

<style lang="scss" scoped>
.user-profile-header {
  display: flex;
  align-items: center;
}

.user-info {
  display: flex;
  align-items: center;
  padding: 6px 10px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
  
  &:hover {
    background-color: var(--header-hover-bg, rgba(255, 255, 255, 0.05));
  }
}

.avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background-color: var(--avatar-bg);
  color: var(--avatar-text);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 500;
  margin-right: 8px;
}

.user-details {
  margin-right: 8px;
}

.username {
  font-size: 14px;
  font-weight: 500;
  color: var(--tab-active-text);
}

.email {
  font-size: 12px;
  color: var(--tab-text);
}

.menu-button {
  background: transparent;
  border: none;
  color: var(--tab-text);
  cursor: pointer;
  padding: 4px;
  
  &:hover {
    color: var(--tab-active-text);
  }
}

.login-button {
  background-color: var(--button-primary);
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  
  &:hover {
    background-color: var(--button-primary-hover);
  }
}
</style>