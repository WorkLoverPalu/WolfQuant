<template>
    <div class="user-scripts">
      <div class="empty-state" v-if="!scripts.length">
        <div class="empty-icon">📝</div>
        <h3>没有脚本</h3>
        <p>用户尚未发布任何脚本</p>
      </div>
      <div v-else>
        <!-- 脚本列表 -->
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  defineProps({
    scripts: {
      type: Array,
      default: () => []
    }
  });
  </script>
  
  <style lang="scss" scoped>
  .user-scripts {
    min-height: 300px;
  }
  
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 0;
    
    .empty-icon {
      font-size: 48px;
      margin-bottom: 16px;
    }
    
    h3 {
      font-size: 18px;
      margin-bottom: 8px;
      color: #d1d4dc;
    }
    
    p {
      font-size: 14px;
      color: #787b86;
    }
  }
  </style>