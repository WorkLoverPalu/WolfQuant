<template>
    <div class="user-stats">
      <div class="stat-item">
        <div class="stat-value">{{ followers }}</div>
        <div class="stat-label">粉丝</div>
      </div>
      
      <div class="stat-item">
        <div class="stat-value">{{ following }}</div>
        <div class="stat-label">正在关注</div>
      </div>
      
      <div class="stat-item">
        <div class="stat-value">{{ views }}</div>
        <div class="stat-label">观点</div>
      </div>
      
      <div class="stat-item">
        <div class="stat-value">{{ scripts }}</div>
        <div class="stat-label">脚本</div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  defineProps({
    followers: {
      type: Number,
      default: 0
    },
    following: {
      type: Number,
      default: 0
    },
    views: {
      type: Number,
      default: 0
    },
    scripts: {
      type: Number,
      default: 0
    }
  });
  </script>
  
  <style lang="scss" scoped>
  .user-stats {
    display: flex;
    margin-bottom: 30px;
  }
  
  .stat-item {
    margin-right: 40px;
    text-align: center;
  }
  
  .stat-value {
    font-size: 24px;
    font-weight: 500;
    margin-bottom: 4px;
  }
  
  .stat-label {
    font-size: 14px;
    color: #787b86;
  }
  </style>