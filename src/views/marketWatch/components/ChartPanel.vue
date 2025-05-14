<template>
    <div class="right-panel">
      <!-- 右侧上部分 - 主图表 -->
      <div class="right-top" :style="{ height: `${rightTopHeight}px` }">
        <ChartHeader :selectedSymbol="selectedSymbol" />
        <MainChart :selectedSymbol="selectedSymbol" />
      </div>
      
      <!-- 水平分隔线 - 可拖动调整高度 -->
      <div class="horizontal-resizer" @mousedown="$emit('startResizeHorizontalRight', $event)"></div>
      
      <!-- 右侧下部分 - 副图表 -->
      <div class="right-bottom">
        <SubChartTabs />
        <SubChart :selectedSymbol="selectedSymbol" />
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { inject } from 'vue';
  import ChartHeader from './chart/ChartHeader.vue';
  import MainChart from './chart/MainChart.vue';
  import SubChartTabs from './chart/SubChartTabs.vue';
  import SubChart from './chart/SubChart.vue';
  
  // 接收父组件传递的属性
  const props = defineProps({
    rightTopHeight: Number
  });
  
  // 定义事件
  const emit = defineEmits([
    'startResizeHorizontalRight'
  ]);
  
  // 注入数据
  const selectedSymbol = inject('selectedSymbol');
  </script>
  
  <style lang="scss" scoped>
  /* 右侧面板 */
  .right-panel {
    width: 650px;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  
  .right-top {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  
  .right-bottom {
    flex: 1;
    border-top: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  
  /* 水平分隔线 */
  .horizontal-resizer {
    width: 100%;
    height: 5px;
    background-color: var(--bg-color);
    cursor: ns-resize;
    position: relative;
    
    &:hover, &:active {
      background-color: var(--resizer-hover-color);
    }
    
    &::after {
      content: '';
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 30px;
      height: 1px;
      background-color: var(--resizer-color);
    }
  }
  </style>
  