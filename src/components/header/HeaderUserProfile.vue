<template>
  <div class="user-profile">
    <!-- 已登录状态 -->
    <div v-if="userStore.isAuthenticated" class="user-logged-in" @click="$emit('open-menu')">
      <div class="user-avatar">
        {{ userStore.user?.avatar || userStore.userInitial }}
      </div>
      <!-- <span class="username">{{ userStore.username }}</span> -->
    </div>

    <!-- 未登录状态 -->
    <div v-else class="user-logged-out" @click="$emit('login')">
      <div class="user-icon">
        <UserIcon />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import UserIcon from '../../assets/icons/UserIcon.vue';
import { useUserStore } from "../../stores/userStore";

// 获取 userStore 实例
const userStore = useUserStore();

defineEmits<{
  (e: 'login'): void;
  (e: 'open-menu'): void;
}>();
</script>

<style lang="scss" scoped>
.user-profile {
  display: flex;
  align-items: center;
  cursor: pointer;
  padding: 8px;
}

.user-logged-in,
.user-logged-out {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;

  &:hover {
    opacity: 0.8;
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

.user-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--tab-text);
  width: 24px;
  height: 24px;
}

.username,
.login-text {
  font-size: 12px;
  color: var(--tab-active-text);
}

.login-text {
  color: #3b82f6;
}
</style>