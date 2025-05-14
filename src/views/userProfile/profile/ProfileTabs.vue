<template>
    <div class="profile-tabs">
      <button 
        v-for="tab in tabs" 
        :key="tab.id"
        class="tab-button"
        :class="{ active: activeTab === tab.id }"
        @click="$emit('change-tab', tab.id)"
      >
        {{ tab.label }}
      </button>
    </div>
  </template>
  
  <script setup lang="ts">
  import { defineProps, defineEmits } from 'vue';
  
  defineProps({
    activeTab: {
      type: String,
      required: true
    }
  });
  
  defineEmits(['change-tab']);
  
  const tabs = [
    { id: 'views', label: '观点' },
    { id: 'methods', label: '看法' },
    { id: 'scripts', label: '脚本' },
    { id: 'followers', label: '粉丝' },
    { id: 'following', label: '正在关注' },
    { id: 'settings', label: '设置' }
  ];
  </script>
  
  <style lang="scss" scoped>
  .profile-tabs {
    display: flex;
    border-bottom: 1px solid #2a2e39;
    margin-bottom: 20px;
  }
  
  .tab-button {
    padding: 12px 16px;
    background: transparent;
    border: none;
    color: #787b86;
    font-size: 14px;
    cursor: pointer;
    position: relative;
    
    &:hover {
      color: #d1d4dc;
    }
    
    &.active {
      color: #d1d4dc;
      
      &::after {
        content: '';
        position: absolute;
        bottom: -1px;
        left: 0;
        width: 100%;
        height: 2px;
        background-color: #2962ff;
      }
    }
  }
  </style>