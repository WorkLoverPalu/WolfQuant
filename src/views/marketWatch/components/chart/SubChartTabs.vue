<template>
    <div class="sub-chart-tabs">
      <button class="sub-chart-tab active">成交量</button>
      <button class="sub-chart-tab">MACD</button>
      <button class="sub-chart-tab">RSI</button>
      <button class="sub-chart-tab">KDJ</button>
      <button class="sub-chart-tab">
        <PlusIcon class="icon-tiny" />
        <span>添加指标</span>
      </button>
    </div>
  </template>
  
  <script setup lang="ts">
  import { PlusIcon } from 'lucide-vue-next';
  </script>
  
  <style lang="scss" scoped>
  .sub-chart-tabs {
    display: flex;
    align-items: center;
    padding: 0 16px;
    height: 36px;
    background-color: var(--headerBg);
    border-bottom: 1px solid var(--borderColor);
    overflow-x: auto;
    
    &::-webkit-scrollbar {
      height: 0;
    }
  }
  
  .sub-chart-tab {
    padding: 0 12px;
    height: 36px;
    background: transparent;
    border: none;
    color: var(--textSecondary);
    font-size: 12px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 4px;
    
    &:hover {
      color: var(--textColor);
    }
    
    &.active {
      color: var(--accentColor);
      position: relative;
      
      &::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 0;
        width: 100%;
        height: 2px;
        background-color: var(--accentColor);
      }
    }
    
    .icon-tiny {
      width: 12px;
      height: 12px;
    }
  }
  </style>
  