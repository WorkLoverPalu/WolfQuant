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
              <button class="theme-option" :class="{ active: themeContext.currentTheme === 'system' }"
                @click="setTheme('system')">
                <div class="theme-preview system-theme"></div>
                <span>系统</span>
              </button>
              <button class="theme-option" :class="{ active: themeContext.currentTheme === 'dark' }"
                @click="setTheme('dark')">
                <div class="theme-preview dark-theme"></div>
                <span>暗色</span>
              </button>
              <button class="theme-option" :class="{ active: themeContext.currentTheme === 'light' }"
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
import { ref, reactive, inject } from 'vue';
import type { ThemeType } from '../../services/theme-service';
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

// 注入主题上下文
const themeContext = inject('theme', {
  currentTheme: 'dark',
  setTheme: (theme: ThemeType) => { }
});

// 设置主题
const setTheme = (theme: ThemeType) => {
  themeContext.setTheme(theme);
  settings.theme = theme;
};

const settings = reactive({
  theme: themeContext.currentTheme,
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
  z-index: 1000;
}

.settings-modal {
  background-color: var(--modal-bg);
  border-radius: 8px;
  width: 800px;
  height: 600px;
  max-width: 90vw;
  max-height: 90vh;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid var(--border-color);

  h2 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
  }

  .close-button {
    background: transparent;
    border: none;
    color: var(--tab-text);
    font-size: 24px;
    cursor: pointer;

    &:hover {
      color: var(--tab-active-text);
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
  border-right: 1px solid var(--border-color);
  overflow-y: auto;

  .sidebar-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 12px 16px;
    background: transparent;
    border: none;
    color: var(--tab-text);
    text-align: left;
    cursor: pointer;

    &:hover {
      background-color: rgba(255, 255, 255, 0.05);
    }

    &.active {
      background-color: rgba(255, 255, 255, 0.1);
      color: var(--tab-active-text);
    }

    svg {
      width: 20px;
      height: 20px;
      margin-right: 12px;
    }
  }
}

.settings-main {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

.settings-section {
  h3 {
    font-size: 14px;
    font-weight: 500;
    color: var(--tab-text);
    margin-bottom: 16px;
    margin-top: 24px;

    &:first-child {
      margin-top: 0;
    }
  }
}

.settings-option {
  margin-bottom: 16px;
}

.checkbox-container {
  display: flex;
  align-items: center;
  cursor: pointer;

  input[type="checkbox"] {
    margin-right: 8px;
  }

  .checkbox-label {
    font-size: 14px;
  }
}

.theme-options {
  display: flex;
  gap: 16px;
  margin-bottom: 24px;
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
      border: 2px solid var(--button-primary);
    }
  }

  .theme-preview {
    width: 80px;
    height: 50px;
    border-radius: 4px;
    margin-bottom: 8px;
    border: 1px solid var(--border-color);

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

.download-path {
  display: flex;
  align-items: center;
  margin-bottom: 16px;

  .path-display {
    flex: 1;
    padding: 8px 12px;
    background-color: rgba(255, 255, 255, 0.05);
    border-radius: 4px;
    font-size: 14px;
    color: var(--tab-text);
  }

  .browse-button {
    margin-left: 8px;
    padding: 8px 16px;
    background-color: transparent;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--tab-active-text);
    cursor: pointer;

    &:hover {
      background-color: rgba(255, 255, 255, 0.05);
    }
  }
}

.volume-control {
  display: flex;
  align-items: center;
  gap: 12px;

  .volume-icon {
    color: var(--tab-text);
  }

  .volume-slider {
    flex: 1;
    height: 4px;
    -webkit-appearance: none;
    background: var(--border-color);
    border-radius: 2px;
    outline: none;

    &::-webkit-slider-thumb {
      -webkit-appearance: none;
      width: 16px;
      height: 16px;
      border-radius: 50%;
      background: var(--tab-active-text);
      cursor: pointer;
    }
  }
}

.about-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 20px 0;

  .app-logo {
    width: 80px;
    height: 80px;
    background-color: #000;
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 16px;

    .logo-image {
      width: 60%;
      height: auto;
    }
  }

  h3 {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 8px;
  }

  .version-info {
    font-size: 14px;
    color: var(--tab-text);
    margin-bottom: 24px;
  }

  .update-button {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 10px 20px;
    background-color: rgba(255, 255, 255, 0.1);
    border: none;
    border-radius: 4px;
    color: var(--tab-active-text);
    font-size: 14px;
    cursor: pointer;
    margin-bottom: 24px;

    &:hover {
      background-color: rgba(255, 255, 255, 0.15);
    }

    .update-icon {
      margin-right: 8px;
    }
  }

  .about-links {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 24px;

    .about-link {
      color: var(--button-primary);
      text-decoration: none;
      font-size: 14px;

      &:hover {
        text-decoration: underline;
      }
    }
  }

  .copyright {
    font-size: 12px;
    color: var(--tab-text);
  }
}
</style>