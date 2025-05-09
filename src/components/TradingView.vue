<template>
    <div class="trading-view">
      <div class="trading-header">
        <h2>{{ symbol }}</h2>
        <div class="price-info">
          <span class="price">{{ price }}</span>
          <span class="change" :class="changeClass">{{ change }}</span>
        </div>
      </div>
      <div class="chart-placeholder">
        <p>这里将显示 {{ symbol }} 的交易图表</p>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { computed } from 'vue';
  
  const props = defineProps<{
    symbol: string;
    price: string;
    change: string;
  }>();
  
  const changeClass = computed(() => {
    if (props.change.startsWith('+')) return 'positive';
    if (props.change.startsWith('-')) return 'negative';
    return '';
  });
  </script>
  
  <style lang="scss" scoped>
  .trading-view {
    height: 100%;
    padding: 16px;
  }
  
  .trading-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    
    h2 {
      font-size: 18px;
      font-weight: 600;
    }
  }
  
  .price-info {
    display: flex;
    align-items: center;
    gap: 10px;
    
    .price {
      font-size: 16px;
      font-weight: 600;
    }
    
    .change {
      font-size: 14px;
      
      &.positive {
        color: #4caf50;
      }
      
      &.negative {
        color: #f44336;
      }
    }
  }
  
  .chart-placeholder {
    height: calc(100% - 60px);
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
  }
  </style>