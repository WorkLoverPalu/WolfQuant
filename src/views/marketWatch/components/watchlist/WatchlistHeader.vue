<template>
    <div class="watchlist-header">
      <div class="category-tabs">
        <button 
          v-for="category in categories" 
          :key="category.id"
          class="category-tab"
          :class="{ active: activeCategory === category.id }"
          @click="$emit('setActiveCategory', category.id)"
        >
          {{ category.name }}
        </button>
      </div>
      
      <div class="header-actions">
        <button class="action-button" @click="$emit('openAddGroupModal')" title="新增分组">
          <PlusIcon class="icon-small" />
        </button>
        
        <div class="sort-dropdown">
          <button class="sort-button" @click="$emit('toggleSortMenu')" title="排序选项">
            <ArrowUpDownIcon class="icon-small" />
            <ChevronDownIcon class="icon-tiny" />
          </button>
          <div v-if="showSortMenu" class="sort-menu">
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
        
        <button class="action-button" @click="$emit('toggleChartView')" title="切换视图">
          <BarChartIcon v-if="!showChartView" class="icon-small" />
          <ListIcon v-else class="icon-small" />
        </button>
        
        <button class="action-button" @click="$emit('openPositionSettingsModal')" title="持仓设置">
          <WalletIcon class="icon-small" />
        </button>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { 
    PlusIcon, 
    ArrowUpDownIcon,
    ChevronDownIcon,
    BarChartIcon,
    ListIcon,
    WalletIcon
  } from 'lucide-vue-next';
  
  // 接收父组件传递的属性
  const props = defineProps({
    categories: Array,
    activeCategory: String,
    sortOptions: Array,
    currentSort: String,
    showSortMenu: Boolean,
    showChartView: Boolean
  });
  
  // 定义事件
  const emit = defineEmits([
    'setActiveCategory',
    'toggleSortMenu',
    'setSort',
    'toggleChartView',
    'openPositionSettingsModal',
    'openAddGroupModal'
  ]);
  </script>
  
  <style lang="scss" scoped>
  /* 顶部导航栏 */
  .watchlist-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 16px;
    height: 48px;
    background-color: var(--headerBg);
    border-bottom: 1px solid var(--borderColor);
    flex-shrink: 0;
  }
  
  .category-tabs {
    display: flex;
    gap: 8px;
  }
  
  .category-tab {
    padding: 6px 12px;
    border: none;
    background: transparent;
    color: var(--textSecondary);
    font-size: 13px;
    cursor: pointer;
    border-radius: 4px;
    
    &:hover {
      background-color: var(--hover-bg);
      color: var(--textColor);
    }
    
    &.active {
      background-color: var(--activeBg);
      color: var(--textColor);
      font-weight: 500;
    }
  }
  
  .header-actions {
    display: flex;
    gap: 4px;
  }
  
  .action-button {
    width: 28px;
    height: 28px;
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
    
    .icon-small {
      width: 16px;
      height: 16px;
    }
    
    .icon-tiny {
      width: 14px;
      height: 14px;
    }
  }
  
  /* 排序下拉菜单 */
  .sort-dropdown {
    position: relative;
  }
  
  .sort-button {
    width: 28px;
    height: 28px;
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
  }
  
  .sort-menu {
    position: absolute;
    top: 100%;
    right: 0;
    width: 150px;
    background-color: var(--cardBg);
    border: 1px solid var(--borderColor);
    border-radius: 4px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
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
    color: var(--textColor);
    font-size: 13px;
    cursor: pointer;
    
    &:hover {
      background-color: var(--hover-bg);
    }
    
    &.active {
      background-color: var(--activeBg);
      font-weight: 500;
    }
  }
  </style>
  