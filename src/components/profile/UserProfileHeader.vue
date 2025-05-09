<template>
    <div class="user-profile-header">
      <div class="avatar-container">
        <div class="avatar" :style="{ backgroundColor: getAvatarColor() }">
          {{ avatar }}
        </div>
        <div class="online-indicator" v-if="isOnline"></div>
      </div>
      
      <div class="user-info">
        <div class="username-container">
          <h1 class="username">{{ username }}</h1>
          <span class="online-status" v-if="isOnline">在线</span>
        </div>
        <div class="join-date">
          <img src="@/assets/tradingview-logo.svg" alt="TradingView" class="tv-logo" />
          <span>{{ joinDate }}</span>
        </div>
      </div>
      
      <div class="actions">
        <button class="edit-profile-btn">
          <i class="icon-edit"></i>
          个人资料设置
        </button>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  defineProps({
    username: {
      type: String,
      required: true
    },
    avatar: {
      type: String,
      required: true
    },
    isOnline: {
      type: Boolean,
      default: false
    },
    joinDate: {
      type: String,
      required: true
    }
  });
  
  // 根据用户名生成一个一致的颜色
  const getAvatarColor = () => {
    return '#b79b6c'; // 固定颜色，与图片一致
  };
  </script>
  
  <style lang="scss" scoped>
  .user-profile-header {
    display: flex;
    align-items: flex-start;
    margin-bottom: 24px;
  }
  
  .avatar-container {
    position: relative;
    margin-right: 20px;
  }
  
  .avatar {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 60px;
    font-weight: bold;
    color: white;
  }
  
  .online-indicator {
    position: absolute;
    bottom: 10px;
    right: 10px;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background-color: #4caf50;
    border: 2px solid #131722;
  }
  
  .user-info {
    flex: 1;
    padding-top: 10px;
  }
  
  .username-container {
    display: flex;
    align-items: center;
    margin-bottom: 8px;
  }
  
  .username {
    font-size: 28px;
    font-weight: 500;
    margin: 0;
    margin-right: 10px;
  }
  
  .online-status {
    font-size: 12px;
    color: #4caf50;
    padding: 2px 6px;
    border-radius: 4px;
    background-color: rgba(76, 175, 80, 0.1);
  }
  
  .join-date {
    display: flex;
    align-items: center;
    font-size: 14px;
    color: #787b86;
  }
  
  .tv-logo {
    width: 20px;
    height: 20px;
    margin-right: 6px;
  }
  
  .actions {
    margin-top: 10px;
  }
  
  .edit-profile-btn {
    background-color: transparent;
    border: 1px solid #5d606b;
    color: #d1d4dc;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    font-size: 14px;
    
    &:hover {
      background-color: rgba(255, 255, 255, 0.05);
    }
    
    .icon-edit {
      margin-right: 6px;
      font-size: 16px;
    }
  }
  </style>