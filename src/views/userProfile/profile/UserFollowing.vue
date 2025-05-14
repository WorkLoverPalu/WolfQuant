<template>
    <div class="user-following">
      <div class="empty-state" v-if="!following.length">
        <div class="empty-icon">👤</div>
        <h3>没有关注</h3>
        <p>用户尚未关注任何人</p>
      </div>
      <div v-else>
        <!-- 关注列表 -->
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  defineProps({
    following: {
      type: Array,
      default: () => []
    }
  });
  </script>
  
  <style lang="scss" scoped>
  .user-following {
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