<template>
  <div class="watchlist-header">
    <!-- 左侧分类选项 -->
    <div class="category-tabs">
      <button
        v-for="type in assetTypes"
        :key="type.id"
        class="category-tab"
        :class="{ active: activeCategory === mapAssetTypeToCategory(type.name) }"
        @click="$emit('setActiveCategory', mapAssetTypeToCategory(type.name))"
      >
        {{ type.description || type.name }}
      </button>
      <button
        class="category-tab"
        :class="{ active: activeCategory === 'all' }"
        @click="$emit('setActiveCategory', 'all')"
      >
        全部
      </button>
    </div>

    <!-- 右侧操作按钮 -->
    <div class="header-actions">
      <!-- 资产类型设置按钮 -->
      <button
        v-if="activeCategory !== 'all'"
        class="action-button"
        @click="$emit('openAssetTypeSettings', getCurrentAssetTypeId())"
        title="资产类型设置"
      >
        <Boxes />
      </button>
      <!-- 排序按钮 -->
      <div class="sort-dropdown">
        <button
          class="action-button"
          @click="$emit('toggleSortMenu')"
          title="排序"
        >
          <SortAscIcon v-if="currentSort.includes('asc')" />
          <SortDescIcon v-else-if="currentSort.includes('desc')" />
          <ArrowDownUp v-else />
        </button>
        <div v-show="showSortMenu" class="sort-menu">
          <button
            v-for="option in sortOptions"
            :key="option.value"
            class="sort-option"
            :class="{ active: currentSort === option.value }"
            @click="$emit('setSort', option.value)"
          >
            {{ option.label }}
          </button>
        </div>
      </div>

      <!-- 视图切换按钮 -->
      <button
        class="action-button"
        @click="$emit('toggleChartView')"
        :title="showChartView ? '切换到列表视图' : '切换到图表视图'"
      >
        <LayoutGridIcon v-if="!showChartView" />
        <ListIcon v-else />
      </button>

      <!-- 持仓设置按钮 -->
      <button
        class="action-button"
        @click="$emit('openPositionSettingsModal')"
        title="持仓设置"
      >
        <WalletIcon />
      </button>
      <!-- 刷新 -->
      <button
        class="action-button"
        @click="$emit('refresh')"
        title="刷新"
      >
        <RefreshCcwDot />
      </button>

      <!-- 添加分组按钮 -->
      <button
        class="action-button add-button"
        @click="$emit('openAddGroupModal')"
        title="添加分组"
      >
        <PlusIcon />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  ArrowDownUp,
  SortAscIcon,
  SortDescIcon,
  LayoutGridIcon,
  ListIcon,
  PlusIcon,
  WalletIcon,
  Boxes,
  RefreshCcwDot
} from 'lucide-vue-next';
import { AssetType } from '../../../../stores/assetStore';

// 接收父组件传递的属性
const props = defineProps<{
  assetTypes: AssetType[];
  activeCategory: string;
  sortOptions: { value: string; label: string }[];
  currentSort: string;
  showSortMenu: boolean;
  showChartView: boolean;
}>();

// 定义事件
const emit = defineEmits([
  'setActiveCategory',
  'toggleSortMenu',
  'setSort',
  'toggleChartView',
  'openPositionSettingsModal',
  'openAddGroupModal',
  'openAssetTypeSettings',
  'refresh'
]);

// 将资产类型代码映射到前端分类
const mapAssetTypeToCategory = (assetTypeName: string): string => {
  const mapping: Record<string, string> = {
    'FUND': 'fund',
    'STOCK': 'stock',
    'GOLD': 'gold',
    'CRYPTO': 'crypto'
  };

  return mapping[assetTypeName] || 'other';
};

// 获取当前资产类型ID
const getCurrentAssetTypeId = (): number => {
  if (props.activeCategory === 'all') return 0;
  
  // 根据当前激活的分类找到对应的资产类型ID
  for (const type of props.assetTypes) {
    if (mapAssetTypeToCategory(type.name) === props.activeCategory) {
      return type.id;
    }
  }
  
  return 0;
};

</script>

<style lang="scss" scoped>
/* 顶部导航栏 */
.watchlist-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--borderColor);
  background-color: var(--headerBg);
}

/* 分类选项卡 */
.category-tabs {
  display: flex;
  gap: 8px;
  overflow-x: auto;
  padding-bottom: 4px;
  
  &::-webkit-scrollbar {
    height: 2px;
  }
  
  &::-webkit-scrollbar-track {
    background: transparent;
  }
  
  &::-webkit-scrollbar-thumb {
    background: var(--scrollbarThumb);
    border-radius: 1px;
  }
}

.category-tab {
  padding: 6px 12px;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  background-color: transparent;
  border: none;
  color: var(--textSecondary);
  cursor: pointer;
  white-space: nowrap;
  
  &:hover {
    background-color: var(--hover-bg);
    color: var(--textColor);
  }
  
  &.active {
    background-color: var(--accentColor);
    color: white;
  }
}

/* 右侧操作按钮 */
.header-actions {
  display: flex;
  gap: 4px;
}

.action-button {
  width: 32px;
  height: 32px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--textSecondary);
  cursor: pointer;
  
  &:hover {
    background-color: var(--hover-bg);
    color: var(--textColor);
  }
  
  svg {
    width: 18px;
    height: 18px;
  }
  
  &.add-button {
    background-color: var(--accentColor);
    color: white;
    
    &:hover {
      background-color: var(--accentColorHover);
    }
  }
}

/* 排序下拉菜单 */
.sort-dropdown {
  position: relative;
}

.sort-menu {
  position: absolute;
  top: 100%;
  right: 0;
  width: 140px;
  background-color: var(--cardBg);
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  z-index: 10;
  margin-top: 4px;
  overflow: hidden;
}

.sort-option {
  width: 100%;
  padding: 8px 12px;
  text-align: left;
  background: transparent;
  border: none;
  font-size: 13px;
  color: var(--textColor);
  cursor: pointer;
  
  &:hover {
    background-color: var(--hover-bg);
  }
  
  &.active {
    color: var(--accentColor);
    font-weight: 500;
  }
}
</style>