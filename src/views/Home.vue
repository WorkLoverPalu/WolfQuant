<template>
    <div class="home-container">
      <h1>欢迎使用交易平台</h1>
      <p>这是首页内容，您可以通过点击右上角的用户头像进入个人中心。</p>
    </div>
  </template>
  
  <script setup lang="ts">
  // 首页组件
  </script>
  
  <style lang="scss" scoped>
  .home-container {
    padding: 40px;
    text-align: center;
    
    h1 {
      margin-bottom: 20px;
      color: var(--tab-active-text);
    }
    
    p {
      color: var(--tab-text);
      font-size: 16px;
    }
  }
  </style>