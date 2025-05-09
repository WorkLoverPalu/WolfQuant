<template>
    <div class="empty-tab">
      <h2>新标签页</h2>
      <p>这是新标签页的占位内容。</p>
    </div>
  </template>
  
  <script setup lang="ts">
  // 空白标签页不需要特殊逻辑
  </script>
  
  <style lang="scss" scoped>
  .empty-tab {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 20px;
    color: var(--tab-text);
    
    h2 {
      margin-bottom: 10px;
    }
  }
  </style>