<template>
    <div 
      class="tab-item" 
      :class="{ 'active': active }"
      @click="$emit('click')"
    >
      <div class="tab-content">
        <span class="tab-title">{{ tab.title }}</span>
        <span v-if="tab.props?.price" class="tab-price">{{ tab.props.price }}</span>
        <span v-if="tab.props?.change" class="tab-change" :class="getChangeClass(tab.props.change)">
          {{ tab.props.change }}
        </span>
      </div>
      <button 
        v-if="tab.closable" 
        class="close-button"
        @click.stop="$emit('close')"
      >
        <XIcon size="14" />
      </button>
    </div>
  </template>
  
  <script setup lang="ts">
  import { XIcon } from 'lucide-vue-next';
  
  interface TabProps {
    id: string;
    title: string;
    component: string;
    props?: Record<string, any>;
    closable: boolean;
  }
  
  defineProps<{
    tab: TabProps;
    active: boolean;
  }>();
  
  defineEmits<{
    (e: 'click'): void;
    (e: 'close'): void;
  }>();
  
  const getChangeClass = (change: string) => {
    if (change.startsWith('+')) return 'positive';
    if (change.startsWith('-')) return 'negative';
    return '';
  };
  </script>
  
  <style lang="scss" scoped>
  .tab-item {
    display: flex;
    align-items: center;
    height: 100%;
    padding: 0 12px;
    min-width: 120px;
    max-width: 200px;
    background-color: var(--tab-bg);
    color: var(--tab-text);
    border-right: 1px solid var(--border-color);
    cursor: pointer;
    user-select: none;
    position: relative;
    
    &.active {
      background-color: var(--tab-active-bg);
      color: var(--tab-active-text);
    }
    
    &:hover {
      .close-button {
        opacity: 1;
      }
    }
  }
  
  .tab-content {
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
  
  .tab-title {
    font-size: 13px;
    font-weight: 500;
  }
  
  .tab-price {
    font-size: 12px;
  }
  
  .tab-change {
    font-size: 12px;
    
    &.positive {
      color: #4caf50;
    }
    
    &.negative {
      color: #f44336;
    }
  }
  
  .close-button {
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: inherit;
    opacity: 0.5;
    padding: 2px;
    margin-left: 6px;
    border-radius: 50%;
    
    &:hover {
      background-color: rgba(255, 255, 255, 0.1);
    }
  }
  </style>