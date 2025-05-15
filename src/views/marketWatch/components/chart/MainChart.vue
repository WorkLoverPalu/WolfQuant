<template>
    <div class="main-chart">
      <!-- 主图表区域 -->
      <div class="chart-placeholder">
        <div v-if="selectedSymbol" class="chart-content">
          <!-- 模拟图表内容 -->
          <div class="chart-canvas" :class="getChangeClass(selectedSymbol.change)"></div>
        </div>
        <div v-else class="no-symbol-selected">
          <LineChartIcon class="large-icon" />
          <p>请从左侧列表选择商品查看图表</p>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { LineChartIcon } from 'lucide-vue-next';
  
  // 接收父组件传递的属性
  const props = defineProps({
    selectedSymbol: Object
  });
  
  // 获取涨跌类名
  const getChangeClass = (change) => {
    if (!change) return '';
    
    if (change.startsWith('-')) {
      return 'negative';
    } else if (change.startsWith('+') || parseFloat(change.replace(',', '')) > 0) {
      return 'positive';
    }
    return '';
  };
  </script>
  
  <style lang="scss" scoped>
  .main-chart {
    flex: 1;
    overflow: hidden;
  }
  
  .chart-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--bgColor);
  }
  
  .no-symbol-selected {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--textSecondary);
    
    .large-icon {
      width: 48px;
      height: 48px;
      margin-bottom: 16px;
      opacity: 0.5;
    }
    
    p {
      font-size: 14px;
    }
  }
  
  .chart-content {
    width: 100%;
    height: 100%;
  }
  
  .chart-canvas {
    width: 100%;
    height: 100%;
    background-image: 
      linear-gradient(to right, var(--chartGridColor) 1px, transparent 1px),
      linear-gradient(to bottom, var(--chartGridColor) 1px, transparent 1px);
    background-size: 50px 50px;
    position: relative;
    
    &::after {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background-repeat: no-repeat;
      background-position: center;
      background-size: 95% 80%;
    }
    
    &.positive::after {
      background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 50'%3E%3Cpath d='M0,35 C5,32 10,30 15,25 C20,20 25,15 30,25 C35,35 40,40 45,35 C50,30 55,25 60,15 C65,5 70,10 75,15 C80,20 85,25 90,20 C95,15 100,10 100,10' stroke='%234caf50' fill='none' stroke-width='2'/%3E%3C/svg%3E");
    }
    
    &.negative::after {
      background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 50'%3E%3Cpath d='M0,15 C5,18 10,20 15,25 C20,30 25,35 30,25 C35,15 40,10 45,15 C50,20 55,25 60,35 C65,45 70,40 75,35 C80,30 85,25 90,30 C95,35 100,40 100,40' stroke='%23f44336' fill='none' stroke-width='2'/%3E%3C/svg%3E");
    }
  }
  </style>
  