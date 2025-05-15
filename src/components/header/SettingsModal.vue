<template>
  <div class="settings-modal-overlay" @click="$emit('close')">
    <div class="settings-modal" @click.stop>
      <div class="settings-header">
        <h2>设置</h2>
        <button class="close-button" @click="$emit('close')">×</button>
      </div>

      <div class="settings-content">
        <div class="settings-sidebar">
          <button v-for="(section, index) in sections" :key="index" class="sidebar-item"
            :class="{ active: activeSection === section.id }" @click="activeSection = section.id">
            <component :is="section.icon" />
            <span>{{ section.name }}</span>
          </button>
        </div>

        <div class="settings-main">
          <!-- 一般设置 -->
          <div v-if="activeSection === 'general'" class="settings-section">
            <h3>凭证和字字符串</h3>

            <div class="settings-option">
              <label class="checkbox-container">
                <input type="checkbox" v-model="settings.autoFill">
                <span class="checkbox-label">自动填入经纪商凭证</span>
              </label>
            </div>

            <div class="settings-option">
              <label class="checkbox-container">
                <input type="checkbox" v-model="settings.crosshair">
                <span class="checkbox-label">跨图口同步十字线</span>
              </label>
            </div>

            <h3>主题</h3>
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

            <h3>下载</h3>
            <div class="download-path">
              <div class="path-display">/Users/caster/Downloads</div>
              <button class="browse-button">跳转</button>
            </div>

            <div class="settings-option">
              <label class="checkbox-container">
                <input type="checkbox" v-model="settings.askDownloadLocation">
                <span class="checkbox-label">总是询问文件保存位置</span>
              </label>
            </div>
          </div>

          <!-- 标签页设置 -->
          <div v-if="activeSection === 'tabs'" class="settings-section">
            <h3>标签页设置</h3>
            <!-- 标签页设置内容 -->
          </div>

          <!-- 视频和音频设置 -->
          <div v-if="activeSection === 'media'" class="settings-section">
            <h3>视频和音频设置</h3>
            <!-- 视频和音频设置内容 -->
          </div>

          <!-- 警报设置 -->
          <div v-if="activeSection === 'alerts'" class="settings-section">
            <h3>通知</h3>

            <div class="settings-option">
              <label class="checkbox-container">
                <input type="checkbox" v-model="settings.systemNotifications">
                <span class="checkbox-label">使用系统通知进行警报</span>
              </label>
            </div>

            <h3>声音</h3>
            <div class="volume-control">
              <div class="volume-icon">
                <VolumeIcon />
              </div>
              <input type="range" min="0" max="100" v-model="settings.volume" class="volume-slider">
            </div>
          </div>

          <!-- 服务设置 -->
          <div v-if="activeSection === 'services'" class="settings-section">
            <h3>服务设置</h3>
            <!-- 服务设置内容 -->
          </div>

          <!-- 网络设置 -->
          <div v-if="activeSection === 'network'" class="settings-section">
            <h3>网络设置</h3>
            <!-- 网络设置内容 -->
          </div>

          <!-- 关于 -->
          <div v-if="activeSection === 'about'" class="settings-section">
            <div class="about-content">
              <div class="app-logo">
                <img src="/logo.svg" alt="WolfQuant Logo" class="logo-image">
              </div>
              <h3>WolfQuant</h3>
              <p class="version-info">Version 2.10.0 · 2025/4/21</p>

              <button class="update-button">
                <span class="update-icon">↻</span>
                检查更新
              </button>

              <div class="about-links">
                <a href="#" class="about-link">声明&致谢</a>
                <a href="#" class="about-link">网站规则</a>
              </div>

              <p class="copyright">版权 © 2025 WolfQuant, Inc.</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { useThemeStore } from '../../stores/themeStore';
import type { ThemeType } from '../../styles/theme';
import GeneralIcon from '../../assets/icons/GeneralIcon.vue';
import TabsIcon from '../../assets/icons/TabsIcon.vue';
import MediaIcon from '../../assets/icons/MediaIcon.vue';
import AlertsIcon from '../../assets/icons/AlertsIcon.vue';
import ServicesIcon from '../../assets/icons/ServicesIcon.vue';
import NetworkIcon from '../../assets/icons/NetworkIcon.vue';
import AboutIcon from '../../assets/icons/AboutIcon.vue';
import VolumeIcon from '../../assets/icons/VolumeIcon.vue';

const activeSection = ref('general');

const sections = [
  { id: 'general', name: '一般', icon: GeneralIcon },
  { id: 'tabs', name: '标签页', icon: TabsIcon },
  { id: 'media', name: '视频和音频', icon: MediaIcon },
  { id: 'alerts', name: '警报', icon: AlertsIcon },
  { id: 'services', name: '服务', icon: ServicesIcon },
  { id: 'network', name: '网络', icon: NetworkIcon },
  { id: 'about', name: '关于', icon: AboutIcon },
];

// 使用 Pinia theme store
const themeStore = useThemeStore();

// 设置主题
const setTheme = (theme: ThemeType) => {
  themeStore.setTheme(theme);
  settings.theme = theme;
};

const settings = reactive({
  theme: themeStore.currentTheme,
  autoFill: false,
  crosshair: true,
  askDownloadLocation: false,
  systemNotifications: true,
  volume: 75
});

defineEmits<{
  (e: 'close'): void;
  (e: 'save', settings: any): void;
}>();
</script>

<style lang="scss" scoped>
.settings-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: var(--z-index-modal);
}

.settings-modal {
  background-color: var(--modalBg);
  border-radius: var(--radius-md);
  width: 800px;
  height: 600px;
  max-width: 90vw;
  max-height: 90vh;
  box-shadow: var(--shadow-lg);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md);
  border-bottom: 1px solid var(--borderColor);

  h2 {
    margin: 0;
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--textColor);
  }

  .close-button {
    background: transparent;
    border: none;
    color: var(--textSecondary);
    font-size: var(--font-size-2xl);
    cursor: pointer;
    transition: color var(--transition-fast);

    &:hover {
      color: var(--textColor);
    }
  }
}

.settings-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.settings-sidebar {
  width: 200px;
  border-right: 1px solid var(--borderColor);
  overflow-y: auto;

  .sidebar-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: var(--spacing-sm) var(--spacing-md);
    background: transparent;
    border: none;
    color: var(--textSecondary);
    text-align: left;
    cursor: pointer;
    transition: background-color var(--transition-fast), color var(--transition-fast);

    &:hover {
      background-color: var(--hoverBg);
    }

    &.active {
      background-color: var(--activeBg);
      color: var(--textColor);
    }

    svg {
      width: 20px;
      height: 20px;
      margin-right: var(--spacing-md);
    }
  }
}

.settings-main {
  flex: 1;
  padding: var(--spacing-lg);
  overflow-y: auto;
}

.settings-section {
  h3 {
    font-size: var(--font-size-md);
    font-weight: 500;
    color: var(--textSecondary);
    margin-bottom: var(--spacing-md);
    margin-top: var(--spacing-lg);

    &:first-child {
      margin-top: 0;
    }
  }
}

.settings-option {
  margin-bottom: var(--spacing-md);
}

.checkbox-container {
  display: flex;
  align-items: center;
  cursor: pointer;

  input[type="checkbox"] {
    margin-right: var(--spacing-sm);
  }

  .checkbox-label {
    font-size: var(--font-size-md);
    color: var(--textColor);
  }
}

.theme-options {
  display: flex;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-lg);
}

.theme-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  background: transparent;
  border: none;
  cursor: pointer;

  &.active {
    .theme-preview {
      border: 2px solid var(--accentColor);
    }
  }

  .theme-preview {
    width: 80px;
    height: 50px;
    border-radius: var(--radius-sm);
    margin-bottom: var(--spacing-sm);
    border: 1px solid var(--borderColor);
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

.download-path {
  display: flex;
  align-items: center;
  margin-bottom: var(--spacing-md);

  .path-display {
    flex: 1;
    padding: var(--spacing-sm) var(--spacing-md);
    background-color: var(--inputBg);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-md);
    color: var(--textSecondary);
  }

  .browse-button {
    margin-left: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    background-color: transparent;
    border: 1px solid var(--borderColor);
    border-radius: var(--radius-sm);
    color: var(--textColor);
    cursor: pointer;
    transition: background-color var(--transition-fast);

    &:hover {
      background-color: var(--hoverBg);
    }
  }
}

.volume-control {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);

  .volume-icon {
    color: var(--textSecondary);
  }

  .volume-slider {
    flex: 1;
    height: 4px;
    -webkit-appearance: none;
    background: var(--borderColor);
    border-radius: 2px;
    outline: none;

    &::-webkit-slider-thumb {
      -webkit-appearance: none;
      width: 16px;
      height: 16px;
      border-radius: 50%;
      background: var(--accentColor);
      cursor: pointer;
    }
  }
}

.about-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: var(--spacing-lg) 0;

  .app-logo {
    width: 80px;
    height: 80px;
    background-color: #000;
    border-radius: var(--radius-lg);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: var(--spacing-md);

    .logo-image {
      width: 60%;
      height: auto;
    }
  }

  h3 {
    font-size: var(--font-size-xl);
    font-weight: 600;
    margin-bottom: var(--spacing-sm);
    color: var(--textColor);
  }

  .version-info {
    font-size: var(--font-size-md);
    color: var(--textSecondary);
    margin-bottom: var(--spacing-lg);
  }

  .update-button {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--spacing-sm) var(--spacing-lg);
    background-color: var(--hoverBg);
    border: none;
    border-radius: var(--radius-sm);
    color: var(--textColor);
    font-size: var(--font-size-md);
    cursor: pointer;
    margin-bottom: var(--spacing-lg);
    transition: background-color var(--transition-fast);

    &:hover {
      background-color: var(--activeBg);
    }

    .update-icon {
      margin-right: var(--spacing-sm);
    }
  }

  .about-links {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    margin-bottom: var(--spacing-lg);

    .about-link {
      color: var(--accentColor);
      text-decoration: none;
      font-size: var(--font-size-md);
      transition: color var(--transition-fast);

      &:hover {
        text-decoration: underline;
      }
    }
  }

  .copyright {
    font-size: var(--font-size-sm);
    color: var(--textSecondary);
  }
}
</style>