<template>
    <div class="sub-chart">
      <!-- 副图表区域 -->
      <div class="chart-placeholder sub">
        <div v-if="selectedSymbol" class="chart-content">
          <!-- 模拟成交量图表 -->
          <div class="volume-chart"></div>
        </div>
        <div v-else class="no-symbol-selected small">
          <BarChart2Icon />
          <p>请选择商品查看指标</p>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { BarChart2Icon } from 'lucide-vue-next';
  
  // 接收父组件传递的属性
  const props = defineProps({
    selectedSymbol: Object
  });
  </script>
  
  <style lang="scss" scoped>
  .sub-chart {
    flex: 1;
    overflow: hidden;
  }
  
  .chart-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--bg-color);
    
    &.sub {
      height: 100%;
    }
  }
  
  .no-symbol-selected {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    
    &.small {
      svg {
        width: 32px;
        height: 32px;
        margin-bottom: 8px;
        opacity: 0.5;
      }
      
      p {
        font-size: 12px;
      }
    }
  }
  
  .chart-content {
    width: 100%;
    height: 100%;
  }
  
  .volume-chart {
    width: 100%;
    height: 100%;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 50'%3E%3Crect x='5' y='30' width='3' height='15' fill='%234caf50' /%3E%3Crect x='10' y='25' width='3' height='20' fill='%234caf50' /%3E%3Crect x='15' y='35' width='3' height='10' fill='%23f44336' /%3E%3Crect x='20' y='20' width='3' height='25' fill='%234caf50' /%3E%3Crect x='25' y='15' width='3' height='30' fill='%234caf50' /%3E%3Crect x='30' y='25' width='3' height='20' fill='%23f44336' /%3E%3Crect x='35' y='30' width='3' height='15' fill='%23f44336' /%3E%3Crect x='40' y='35' width='3' height='10' fill='%23f44336' /%3E%3Crect x='45' y='25' width='3' height='20' fill='%234caf50' /%3E%3Crect x='50' y='20' width='3' height='25' fill='%234caf50' /%3E%3Crect x='55' y='15' width='3' height='30' fill='%234caf50' /%3E%3Crect x='60' y='10' width='3' height='35' fill='%234caf50' /%3E%3Crect x='65' y='20' width='3' height='25' fill='%23f44336' /%3E%3Crect x='70' y='25' width='3' height='20' fill='%23f44336' /%3E%3Crect x='75' y='30' width='3' height='15' fill='%23f44336' /%3E%3Crect x='80' y='25' width='3' height='20' fill='%234caf50' /%3E%3Crect x='85' y='20' width='3' height='25' fill='%234caf50' /%3E%3Crect x='90' y='25' width='3' height='20' fill='%23f44336' /%3E%3Crect x='95' y='30' width='3' height='15' fill='%23f44336' /%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: center;
    background-size: 95% 90%;
  }
  </style>
  