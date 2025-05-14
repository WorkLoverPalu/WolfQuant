<template>
    <div class="user-followers">
      <div class="empty-state" v-if="!followers.length">
        <div class="empty-icon">👥</div>
        <h3>没有粉丝</h3>
        <p>用户尚未有任何粉丝</p>
      </div>
      <div v-else>
        <!-- 粉丝列表 -->
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  defineProps({
    followers: {
      type: Array,
      default: () => []
    }
  });
  </script>
  
  <style lang="scss" scoped>
  .user-followers {
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