<template>
  <div class="user-menu-overlay" @click="$emit('close')">
    <div class="user-menu" @click.stop>
      <div class="user-info" @click="$emit('open-profile')">
        <div class="user-avatar">{{ userInitial }}</div>
        <div class="user-details">
          <div class="username">{{ userStore.user?.username }}</div>
          <div class="email">{{ userStore.user?.email }}</div>
        </div>
      </div>

      <div class="menu-divider"></div>

      <div class="menu-items">
        <button class="menu-item" @click="$emit('open-settings')">
          <SettingsIcon />
          <span>设置</span>
          <span class="shortcut">⌘,</span>
        </button>

        <button class="menu-item" @click="tabStore.addNewTab()">
          <PlusIcon />
          <span>新标签页</span>
          <span class="shortcut">⌘T</span>
        </button>

        <button class="menu-item">
          <WindowIcon />
          <span>新窗口</span>
          <span class="shortcut">⌘N</span>
        </button>

        <button class="menu-item">
          <ClipboardIcon />
          <span>从剪贴板打开链接</span>
        </button>
      </div>

      <div class="menu-divider"></div>

      <div class="menu-items">
        <button class="menu-item">
          <ZoomIcon />
          <span>缩放</span>
          <div class="zoom-controls">
            <button class="zoom-button">−</button>
            <span class="zoom-level">100%</span>
            <button class="zoom-button">+</button>
          </div>
        </button>
      </div>

      <div class="menu-divider"></div>

      <div class="theme-selector">
        <div class="theme-options">
          <button class="theme-option" :class="{ active: themeStore.currentTheme === 'system' }"
            @click="setTheme('system')">
            <div class="theme-preview system-theme"></div>
            <span>系统</span>
          </button>
          <button class="theme-option" :class="{ active: themeStore.currentTheme === 'dark' }"
            @click="setTheme('dark')">
            <div class="theme-preview dark-theme"></div>
            <span>暗色</span>
          </button>
          <button class="theme-option" :class="{ active: themeStore.currentTheme === 'light' }"
            @click="setTheme('light')">
            <div class="theme-preview light-theme"></div>
            <span>亮色</span>
          </button>
        </div>
      </div>

      <div class="menu-divider"></div>

      <button class="menu-item logout-button" @click="handleLogout">
        <LogoutIcon />
        <span>退出 WolfQuant</span>
      </button>
    </div>
  </div>

</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useUserStore } from '../../stores/userStore';
import { useThemeStore } from '../../stores/themeStore';
import { useTabStore } from '../../stores/tabStore';

import type { ThemeType } from '../../styles/theme';
import SettingsIcon from '../../assets/icons/SettingsIcon.vue';
import LogoutIcon from '../../assets/icons/LogoutIcon.vue';
import PlusIcon from '../../assets/icons/PlusIcon.vue';
import WindowIcon from '../../assets/icons/WindowIcon.vue';
import ClipboardIcon from '../../assets/icons/ClipboardIcon.vue';
import ZoomIcon from '../../assets/icons/ZoomIcon.vue';

const userStore = useUserStore();
const themeStore = useThemeStore();
const tabStore = useTabStore();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'open-settings'): void;
  (e: 'open-profile'): void;
}>();

// 设置主题
const setTheme = (theme: ThemeType) => {
  themeStore.setTheme(theme);
};


// 用户头像显示的首字母
const userInitial = computed(() => {
  return userStore.user?.avatar || userStore.user?.username?.charAt(0)?.toUpperCase() || '';
});

// 处理退出登录
const handleLogout = async () => {
  try {
    // 使用 store 的 logout 方法
    await userStore.logout();

    // 通知父组件退出登录
    emit('close');
  } catch (err) {
    console.error('Logout failed:', err);
  }
};
</script>
<style lang="scss" scoped>
.user-menu-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: calc(var(--z-index-dropdown) - 1);
}

.user-menu {
  position: absolute;
  top: 50px;
  right: 50px;
  width: 260px;
  background-color: var(--modalBg);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-md);
  z-index: var(--z-index-dropdown);
  overflow: hidden;
  outline: solid 1px var(--borderColor);
}


.menu-divider {
  height: 1px;
  background-color: var(--borderColor);
  margin: var(--spacing-xs) 0;
}

.user-info {
  display: flex;
  align-items: center;
  padding: var(--spacing-sm) var(--spacing-md);
  cursor: pointer;
  transition: background-color var(--transition-fast);

  &:hover {
    background-color: var(--hoverBg);
  }

  .user-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background-color: var(--cardBg);
    border: solid 1px var(--borderColor);
    color: var(--textColor);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: var(--font-size-xl);
    margin-right: var(--spacing-md);
  }

  .user-details {
    flex: 1;

    .username {
      font-weight: 500;
      font-size: var(--font-size-md);
      margin-bottom: var(--spacing-xs);
      color: var(--textColor);
    }

    .email {
      font-size: var(--font-size-sm);
      color: var(--textSecondary);
    }
  }
}

.menu-items {
  padding: var(--spacing-sm) 0;
}

.menu-item {
  display: flex;
  align-items: center;
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background: transparent;
  border: none;
  color: var(--textColor);
  font-size: var(--font-size-md);
  text-align: left;
  cursor: pointer;
  border-radius: var(--radius-sm);
  transition: background-color var(--transition-fast), color var(--transition-fast);

  &:hover {
    background-color: var(--hoverBg);
  }

  svg {
    margin-right: var(--spacing-md);
    color: var(--textSecondary);
    width: 16px;
    height: 16px;
    transition: color var(--transition-fast);
  }

  span {
    color: var(--textColor);
  }

  .shortcut {
    margin-left: auto;
    font-size: var(--font-size-sm);
    color: var(--textSecondary);
  }

  .zoom-controls {
    margin-left: auto;
    display: flex;
    align-items: center;

    .zoom-button {
      width: 20px;
      height: 20px;
      display: flex;
      align-items: center;
      justify-content: center;
      background: transparent;
      border: none;
      color: var(--textSecondary);
      font-size: var(--font-size-md);
      cursor: pointer;
      transition: color var(--transition-fast);

      &:hover {
        color: var(--textColor);
      }
    }

    .zoom-level {
      margin: 0 var(--spacing-sm);
      font-size: var(--font-size-sm);
      color: var(--textSecondary);
    }
  }

  &.logout-button {
    color: var(--negativeColor);

    svg {
      color: var(--negativeColor);
    }
  }
}

.theme-selector {
  padding: var(--spacing-sm) var(--spacing-md);
}

.theme-options {
  display: flex;
  justify-content: space-between;
  gap: var(--spacing-sm);
}

.theme-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  background: transparent;
  border: none;
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--radius-sm);
  transition: background-color var(--transition-fast);

  &:hover {
    background-color: var(--hoverBg);
  }

  &.active {
    background-color: var(--activeBg);
  }

  .theme-preview {
    width: 64px;
    height: 40px;
    border-radius: var(--radius-sm);
    margin-bottom: var(--spacing-sm);
    border: 1px solid var(--borderColor);
    overflow: hidden;
    transition: border-color var(--transition-fast);

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
    font-size: var(--font-size-sm);
    color: var(--textSecondary);
  }
}
</style>