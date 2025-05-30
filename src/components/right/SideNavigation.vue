<template>
  <div class="side-navigation" :class="{ 'light-theme': theme === 'light' }">
    <!-- 主导航栏 - 右侧固定 -->
    <div class="nav-bar">
      <div v-for="(item, index) in navigationItems" :key="index" class="nav-item"
        :class="{ 'active': activeItem === item.id }" @click="handleItemClick(item)">
        <component :is="item.icon" class="nav-icon" />
        <div v-if="item.badge" class="badge">{{ item.badge }}</div>
      </div>
    </div>

    <!-- 侧边面板 - 在导航栏左侧打开 -->
    <transition name="slide-left">
      <div v-if="activePanelComponent" class="side-panel" :style="{ width: `${panelWidth}px` }">
        <div class="panel-header">
          <h3>{{ activePanelTitle }}</h3>
          <button class="close-button" @click="closePanel">
            <XIcon />
          </button>
        </div>
        <div class="panel-content">
          <component :is="activePanelComponent" @close="closePanel" @open-tab="handleOpenTab" />
        </div>
      </div>
    </transition>

    <!-- 可拖拽边框 - 在面板左侧 -->
    <div v-if="activePanelComponent" class="panel-resizer" :style="{ right: `${48 + panelWidth - 3}px` }"
      @mousedown="startResize"></div>

    <!-- 面板遮罩层 -->
    <div v-if="activePanelComponent" class="panel-backdrop" :style="{ width: `calc(100% - 48px - ${panelWidth}px)` }"
      @click="closePanel"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, shallowRef, inject, onMounted, computed } from 'vue';
import {
  BookmarkIcon,
  ClockIcon,
  MessageCircleIcon,
  TargetIcon,
  CalendarIcon,
  ActivityIcon,
  BarChart2Icon,
  RssIcon,
  BellIcon,
  HelpCircleIcon,
  XIcon
} from 'lucide-vue-next';

// 导入面板组件
import BookmarkPanel from './panels/BookmarkPanel.vue';
import HistoryPanel from './panels/HistoryPanel.vue';
import MessagesPanel from './panels/MessagesPanel.vue';
import WatchlistPanel from './panels/WatchlistPanel.vue';
import CalendarPanel from './panels/CalendarPanel.vue';
import ChartsPanel from './panels/ChartsPanel.vue';
import IndicatorsPanel from './panels/IndicatorsPanel.vue';
import NewsPanel from './panels/NewsPanel.vue';
import NotificationsPanel from './panels/NotificationsPanel.vue';
import HelpPanel from './panels/HelpPanel.vue';

// 定义组件属性
interface Props {
  // 当前用户
  currentUser?: any;
  // 顶部标签栏高度
  headerHeight?: number;
}

const props = withDefaults(defineProps<Props>(), {
  currentUser: null,
  headerHeight: 40
});

// 定义事件
const emit = defineEmits<{
  // 打开新标签页事件
  (e: 'open-tab', tabData: any): void;
}>();

// 从ThemeProvider获取主题
const theme = inject('theme', ref('dark'));

// 当前激活的项目
const activeItem = ref<string | null>(null);
const activePanelComponent = shallowRef<any>(null);
const activePanelTitle = ref('');

// 面板宽度和拖拽状态
const panelWidth = ref(320);
const minPanelWidth = 250;
const maxPanelWidth = computed(() => Math.min(600, window.innerWidth * 0.5));
const isResizing = ref(false);

// 导航项目配置
const navigationItems = ref([
  {
    id: 'bookmark',
    icon: BookmarkIcon,
    title: '收藏',
    panel: BookmarkPanel,
    openInPanel: true
  },
  {
    id: 'history',
    icon: ClockIcon,
    title: '历史',
    panel: HistoryPanel,
    openInPanel: true
  },
  {
    id: 'messages',
    icon: MessageCircleIcon,
    title: '消息',
    panel: MessagesPanel,
    badge: 1,
    openInPanel: true
  },
  {
    id: 'watchlist',
    icon: TargetIcon,
    title: '自选',
    panel: WatchlistPanel,
    openInPanel: true
  },
  {
    id: 'calendar',
    icon: CalendarIcon,
    title: '日历',
    panel: CalendarPanel,
    openInPanel: true
  },
  {
    id: 'charts',
    icon: ActivityIcon,
    title: '图表',
    panel: ChartsPanel,
    openInPanel: false,
    tabData: {
      id: 'charts',
      title: '图表',
      component: 'ChartsTab',
      closable: true
    }
  },
  {
    id: 'indicators',
    icon: BarChart2Icon,
    title: '指标',
    panel: IndicatorsPanel,
    openInPanel: true
  },
  {
    id: 'news',
    icon: RssIcon,
    title: '新闻',
    panel: NewsPanel,
    badge: 1,
    openInPanel: true
  },
  {
    id: 'notifications',
    icon: BellIcon,
    title: '通知',
    panel: NotificationsPanel,
    openInPanel: true
  },
  {
    id: 'help',
    icon: HelpCircleIcon,
    title: '帮助',
    panel: HelpPanel,
    openInPanel: true
  }
]);

// 处理导航项点击
const handleItemClick = (item: any) => {
  // 如果点击当前激活的项目，则关闭面板
  if (activeItem.value === item.id && activePanelComponent.value) {
    closePanel();
    return;
  }

  activeItem.value = item.id;

  // 如果是在面板中打开
  if (item.openInPanel && item.panel) {
    activePanelComponent.value = item.panel;
    activePanelTitle.value = item.title;
  }
  // 否则在新标签页中打开
  else if (!item.openInPanel && item.tabData) {
    emit('open-tab', item.tabData);
    // 不显示面板
    activePanelComponent.value = null;
  }
};

// 处理打开标签页
const handleOpenTab = (tabData: any) => {
  emit('open-tab', tabData);
  // 可选：关闭面板
  // closePanel();
};

// 关闭面板
const closePanel = () => {
  activeItem.value = null;
  activePanelComponent.value = null;
};

// 开始调整面板大小
const startResize = (e: MouseEvent) => {
  isResizing.value = true;

  const handleMouseMove = (e: MouseEvent) => {
    if (isResizing.value) {
      // 计算新宽度，确保在合理范围内
      // 注意：现在是从右向左拖拽，所以计算方式不同
      const navBarRight = window.innerWidth;
      const newWidth = navBarRight - e.clientX - 48; // 减去导航栏宽度
      panelWidth.value = Math.max(minPanelWidth, Math.min(newWidth, maxPanelWidth.value));
    }
  };

  const handleMouseUp = () => {
    isResizing.value = false;
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', handleMouseUp);
  };

  document.addEventListener('mousemove', handleMouseMove);
  document.addEventListener('mouseup', handleMouseUp);

  // 防止拖拽时选中文本
  e.preventDefault();
};

// 更新导航项
const updateNavigationItem = (id: string, updates: Partial<any>) => {
  const itemIndex = navigationItems.value.findIndex(item => item.id === id);
  if (itemIndex !== -1) {
    navigationItems.value[itemIndex] = {
      ...navigationItems.value[itemIndex],
      ...updates
    };
  }
};

// 设置导航项徽章
const setBadge = (id: string, badge: number | null) => {
  updateNavigationItem(id, { badge });
};

// 组件挂载时
onMounted(() => {
  console.log('右侧导航组件已加载');

  // 监听窗口大小变化，确保面板宽度不超过最大值
  window.addEventListener('resize', () => {
    if (panelWidth.value > maxPanelWidth.value) {
      panelWidth.value = maxPanelWidth.value;
    }
  });
});

// 暴露方法给父组件
defineExpose({
  closePanel,
  updateNavigationItem,
  setBadge
});
</script>

<style lang="scss" scoped>
.side-navigation {
  position: fixed;
  top: var(--header-height, 48px);
  right: 0;
  height: calc(100vh - var(--header-height, 48px));
  display: flex;
  flex-direction: row-reverse;
  z-index: var(--z-index-dropdown);
  transition: all var(--transition-normal);
}

.nav-bar {
  width: 48px;
  height: 100%;
  background-color: var(--navBg);
  border-left: 1px solid var(--borderColor);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--spacing-sm) 0;
  z-index: calc(var(--z-index-dropdown) + 1);
}

.nav-item {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: var(--spacing-xs) 0;
  border-radius: var(--radius-sm);
  cursor: pointer;
  color: var(--iconColor);
  position: relative;
  transition: background-color var(--transition-fast), color var(--transition-fast);

  &:hover {
    background-color: var(--hoverBg);
    color: var(--iconHoverColor);
  }

  &.active {
    color: var(--iconActiveColor);
  }
}

.nav-icon {
  width: 20px;
  height: 20px;
}

.badge {
  position: absolute;
  top: -4px;
  right: -4px;
  min-width: 18px;
  height: 18px;
  border-radius: 9px;
  background-color: var(--negativeColor);
  color: white;
  font-size: var(--font-size-xs);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 var(--spacing-xs);
}

.side-panel {
  width: 320px;
  height: 100%;
  background-color: var(--cardBg);
  border-right: 1px solid var(--borderColor);
  display: flex;
  flex-direction: column;
  z-index: var(--z-index-dropdown);
  box-shadow: var(--shadow-md);
}

.panel-header {
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--spacing-md);
  border-bottom: 1px solid var(--borderColor);

  h3 {
    margin: 0;
    font-size: var(--font-size-lg);
    font-weight: 500;
    color: var(--textColor);
  }

  .close-button {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    color: var(--iconColor);
    cursor: pointer;
    transition: background-color var(--transition-fast), color var(--transition-fast);

    &:hover {
      background-color: var(--hoverBg);
      color: var(--textColor);
    }

    svg {
      width: 18px;
      height: 18px;
    }
  }
}

.panel-content {
  flex: 1;
  overflow-y: auto;

  &::-webkit-scrollbar {
    width: 6px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background-color: var(--scrollbarThumb);
    border-radius: 3px;
  }
}

.panel-resizer {
  position: absolute;
  top: 0;
  width: 6px;
  height: 100%;
  cursor: ew-resize;
  z-index: calc(var(--z-index-dropdown) + 2);
  transition: background-color var(--transition-fast);

  &:hover {
    background-color: var(--resizerHoverColor);
  }
}

.panel-backdrop {
  position: fixed;
  top: var(--header-height, 48px);
  left: 0;
  height: calc(100vh - var(--header-height, 48px));
  background-color: rgba(0, 0, 0, 0.5);
  z-index: calc(var(--z-index-dropdown) - 1);
}

/* 动画 */
.slide-left-enter-active,
.slide-left-leave-active {
  transition: transform var(--transition-normal);
}

.slide-left-enter-from,
.slide-left-leave-to {
  transform: translateX(100%);
}

/* 设置顶部高度变量 */
.side-navigation {
  --header-height: v-bind('props.headerHeight + "px"');
}
</style>