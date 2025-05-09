<template>
  <div class="user-menu">
    <div class="menu-header">
      <div class="user-info">
        <div class="avatar">{{ user.avatar || user.username.charAt(0).toUpperCase() }}</div>
        <div class="user-details">
          <div class="username">{{ user.username }}</div>
          <div class="email">{{ user.email }}</div>
        </div>
      </div>
    </div>
    
    <div class="menu-items">
      <button class="menu-item" @click="$emit('open-profile')">
        <i class="icon-user"></i>
        个人中心
      </button>
      <button class="menu-item" @click="$emit('open-settings')">
        <i class="icon-settings"></i>
        设置
      </button>
      <button class="menu-item" @click="$emit('logout')">
        <i class="icon-log-out"></i>
        退出登录
      </button>
    </div>
  </div>
  
  <div class="menu-backdrop" @click="$emit('close')"></div>
</template>

<script setup lang="ts">
defineProps({
  user: {
    type: Object,
    required: true
  }
});

defineEmits(['close', 'logout', 'open-settings', 'open-profile']);
</script>

<style lang="scss" scoped>
.user-menu {
  position: absolute;
  top: 60px;
  right: 20px;
  width: 240px;
  background-color: var(--modal-bg);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  z-index: 1000;
  overflow: hidden;
}

.menu-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 999;
}

.menu-header {
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
}

.user-info {
  display: flex;
  align-items: center;
}

.avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background-color: var(--avatar-bg);
  color: var(--avatar-text);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 500;
  margin-right: 12px;
}

.user-details {
  flex: 1;
}

.username {
  font-size: 16px;
  font-weight: 500;
  color: var(--tab-active-text);
  margin-bottom: 4px;
}

.email {
  font-size: 12px;
  color: var(--tab-text);
}

.menu-items {
  padding: 8px 0;
}

.menu-item {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 12px 16px;
  background: transparent;
  border: none;
  color: var(--tab-active-text);
  font-size: 14px;
  text-align: left;
  cursor: pointer;
  
  i {
    margin-right: 12px;
    font-size: 16px;
  }
  
  &:hover {
    background-color: rgba(255, 255, 255, 0.05);
  }
}
</style>