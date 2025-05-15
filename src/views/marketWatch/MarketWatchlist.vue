<template>
  <div class="market-watchlist" :class="{ 'light-theme': themeStore.currentTheme }">
    <div class="watchlist-container">
      <!-- 左侧面板 -->
      <WatchlistPanel :activeCategory="activeCategory" :leftPanelWidth="leftPanelWidth" :leftTopHeight="leftTopHeight"
        @startResizeVertical="startResizeVertical" />

      <!-- 垂直分隔线 - 可拖动调整宽度 -->
      <div class="vertical-resizer" @mousedown="startResizeVertical"></div>

      <!-- 右侧面板 - 图表区域 -->
      <ChartPanel :rightTopHeight="rightTopHeight" @startResizeHorizontalRight="startResizeHorizontalRight" />
    </div>

    <!-- 添加商品弹窗 -->
    <SymbolModal v-if="showAddSymbolModal" />

    <!-- 添加/编辑分组弹窗 -->
    <GroupModal v-if="showGroupModal" />

    <!-- 持仓设置弹窗 -->
    <PositionModal v-if="showPositionModal" />
  </div>
</template>

<script setup lang="ts">
import { ref, provide, onMounted, onUnmounted } from 'vue';
import WatchlistPanel from './components/WatchlistPanel.vue';
import ChartPanel from './components/ChartPanel.vue';
import SymbolModal from './components/SymbolModal.vue';
import GroupModal from './components/GroupModal.vue';
import PositionModal from './components/PositionModal.vue';

import { useThemeStore } from "../../stores/themeStore";
const themeStore = useThemeStore();


// 当前激活的分类
const activeCategory = ref('fund');

// 模态框状态
const showAddSymbolModal = ref(false);
const showGroupModal = ref(false);
const showPositionModal = ref(false);

// 面板尺寸调整
const leftPanelWidth = ref(350); // 左侧面板宽度
const leftTopHeight = ref(500); // 左侧上部分高度
const rightTopHeight = ref(500); // 右侧上部分高度
const isResizingVertical = ref(false);
const isResizingHorizontalRight = ref(false);

// 开始垂直调整大小（左右拖动）
const startResizeVertical = (e: MouseEvent) => {
  isResizingVertical.value = true;

  const handleMouseMove = (e: MouseEvent) => {
    if (isResizingVertical.value) {
      // 计算新宽度，确保在合理范围内
      const newWidth = e.clientX;
      leftPanelWidth.value = Math.max(250, Math.min(newWidth, window.innerWidth * 0.7));
    }
  };

  const handleMouseUp = () => {
    isResizingVertical.value = false;
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', handleMouseUp);
  };

  document.addEventListener('mousemove', handleMouseMove);
  document.addEventListener('mouseup', handleMouseUp);

  // 防止拖拽时选中文本
  e.preventDefault();
};

// 开始水平调整大小（右侧上下拖动）
const startResizeHorizontalRight = (e: MouseEvent) => {
  isResizingHorizontalRight.value = true;

  const handleMouseMove = (e: MouseEvent) => {
    if (isResizingHorizontalRight.value) {
      // 获取右侧面板的位置信息
      const rightPanel = document.querySelector('.right-panel');
      if (rightPanel) {
        const rect = rightPanel.getBoundingClientRect();
        // 计算新高度，确保在合理范围内
        const newHeight = e.clientY - rect.top;
        rightTopHeight.value = Math.max(200, Math.min(newHeight, window.innerHeight - 100));
      }
    }
  };

  const handleMouseUp = () => {
    isResizingHorizontalRight.value = false;
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', handleMouseUp);
  };

  document.addEventListener('mousemove', handleMouseMove);
  document.addEventListener('mouseup', handleMouseUp);

  // 防止拖拽时选中文本
  e.preventDefault();
};

// 提供全局状态给子组件
provide('showAddSymbolModal', showAddSymbolModal);
provide('showGroupModal', showGroupModal);
provide('showPositionModal', showPositionModal);
provide('activeCategory', activeCategory);

// 初始化
onMounted(() => {
  // 设置初始面板高度
  leftTopHeight.value = window.innerHeight * 0.6;
  rightTopHeight.value = window.innerHeight * 0.6;
});
</script>

<style lang="scss" scoped>

/* 基础样式 */
.market-watchlist {
  width: 100%;
  height: 100vh;
  background-color: var(--bgColor);
  color: var(--textColor);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.watchlist-container {
  display: flex;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

/* 垂直分隔线 */
.vertical-resizer {
  width: 5px;
  height: 100%;
  background-color: var(--bgColor);
  cursor: ew-resize;
  position: relative;

  &:hover,
  &:active {
    background-color: var(--resizerHoverColor);
  }

  &::after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 1px;
    height: 30px;
    background-color: var( --resizerColor);
  }
}
</style>
