<template>
    <div class="settings-tabs">
      <button 
        v-for="tab in tabs" 
        :key="tab.id"
        class="tab-button"
        :class="{ active: activeSettingsTab === tab.id }"
        @click="$emit('change-tab', tab.id)"
      >
        {{ tab.label }}
      </button>
    </div>
  </template>
  
  <script setup lang="ts">
  import { defineProps, defineEmits } from 'vue';
  
  defineProps({
    activeSettingsTab: {
      type: String,
      required: true
    }
  });
  
  defineEmits(['change-tab']);
  
  const tabs = [
    { id: 'general', label: '概览' },
    { id: 'account', label: '账户和账单' },
    { id: 'security', label: '安全' }
  ];
  </script>
  
  <style lang="scss" scoped>
  .settings-tabs {
    display: flex;
    margin-bottom: 20px;
    background-color: #1e222d;
    border-radius: 4px;
    padding: 4px;
  }
  
  .tab-button {
    padding: 8px 16px;
    background: transparent;
    border: none;
    color: #787b86;
    font-size: 14px;
    cursor: pointer;
    border-radius: 4px;
    
    &:hover {
      color: #d1d4dc;
    }
    
    &.active {
      color: #d1d4dc;
      background-color: #2a2e39;
    }
  }
  </style>