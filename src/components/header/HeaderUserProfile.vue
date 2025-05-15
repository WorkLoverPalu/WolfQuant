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
    <div v-else class="user-logged-out" @click="$emit('login', pageId)">
      <div class="user-icon">
        <UserIcon />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import UserIcon from '../../assets/icons/UserIcon.vue';
import { useUserStore } from "../../stores/userStore";
const pageId = 'HeaderUserProfile';
// 获取 userStore 实例
const userStore = useUserStore();

defineEmits<{
  (e: 'login', pageId: string): void;
  (e: 'open-menu'): void;
}>();

</script>

<style lang="scss" scoped>
.user-profile {
  display: flex;
  align-items: center;
  cursor: pointer;
  padding: var(--spacing-sm);
}

.user-logged-in,
.user-logged-out {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-xs);

  &:hover {
    opacity: 0.8;
  }
}

.user-avatar {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background-color: var(--cardBg);
  color: var(--textColor);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
}

.user-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--iconColor);
  width: 24px;
  height: 24px;
}

.username,
.login-text {
  font-size: var(--font-size-xs);
  color: var(--textColor);
}

.login-text {
  color: var(--accentColor);
}
</style>