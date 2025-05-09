<template>
    <div class="user-views">
      <div class="empty-state" v-if="!views.length">
        <div class="empty-icon">📊</div>
        <h3>没有观点</h3>
        <p>用户尚未发布任何观点</p>
      </div>
      <div v-else>
        <!-- 观点列表 -->
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  defineProps({
    views: {
      type: Array,
      default: () => []
    }
  });
  </script>
  
  <style lang="scss" scoped>
  .user-views {
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