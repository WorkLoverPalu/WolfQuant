<template>
    <div class="chart-header">
      <div class="chart-symbol-info" v-if="selectedSymbol">
        <div class="symbol-icon large" :class="getSymbolClass(selectedSymbol.symbol)">
          {{ getSymbolIcon(selectedSymbol.symbol) }}
        </div>
        <div class="symbol-details">
          <div class="symbol-title">{{ selectedSymbol.name }} ({{ selectedSymbol.symbol }})</div>
          <div 
            class="symbol-price"
            :class="getChangeClass(selectedSymbol.change)"
          >
            {{ selectedSymbol.price }}
            <span class="price-change">
              {{ selectedSymbol.change }} ({{ selectedSymbol.changePercent }})
            </span>
          </div>
        </div>
      </div>
      <div class="chart-controls">
        <button class="chart-control-button">
          <LineChartIcon />
          <span>线图</span>
        </button>
        <button class="chart-control-button">
          <BarChartIcon />
          <span>K线图</span>
        </button>
        <button class="chart-control-button">
          <Settings2Icon />
          <span>指标</span>
        </button>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { 
    LineChartIcon, 
    BarChartIcon, 
    Settings2Icon
  } from 'lucide-vue-next';
  
  // 接收父组件传递的属性
  const props = defineProps({
    selectedSymbol: Object
  });
  
  // 获取符号图标
  const getSymbolIcon = (symbol) => {
    const firstChar = symbol.charAt(0);
    return firstChar;
  };
  
  // 获取符号类名
  const getSymbolClass = (symbol) => {
    const symbolMap = {
      'SPX': 'symbol-spx',
      'NDQ': 'symbol-ndq',
      'DJI': 'symbol-dji',
      'VIX': 'symbol-vix',
      'DXY': 'symbol-dxy',
      'BTCUSD': 'symbol-btc',
      'ETHUSD': 'symbol-eth',
      'XAUUSD': 'symbol-gold',
      '518880': 'symbol-fund'
    };
    
    return symbolMap[symbol] || 'symbol-default';
  };
  
  // 获取涨跌类名
  const getChangeClass = (change) => {
    if (change.startsWith('-')) {
      return 'negative';
    } else if (change.startsWith('+') || parseFloat(change.replace(',', '')) > 0) {
      return 'positive';
    }
    return '';
  };
  </script>
  
  <style lang="scss" scoped>
  /* 图表区域 */
  .chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background-color: var(--headerBg);
    border-bottom: 1px solid var(--borderColor);
  }
  
  .chart-symbol-info {
    display: flex;
    align-items: center;
  }
  
  .symbol-icon {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 8px;
    font-size: 10px;
    color: white;
    
    &.large {
      width: 32px;
      height: 32px;
      font-size: 16px;
      margin-right: 12px;
    }
    
    &.symbol-spx {
      background-color: #e91e63;
    }
    
    &.symbol-ndq {
      background-color: #2196f3;
    }
    
    &.symbol-dji {
      background-color: #4caf50;
    }
    
    &.symbol-vix {
      background-color: #ff9800;
    }
    
    &.symbol-dxy {
      background-color: #9c27b0;
    }
    
    &.symbol-btc {
      background-color: #f57c00;
    }
    
    &.symbol-eth {
      background-color: #7b1fa2;
    }
    
    &.symbol-gold {
      background-color: #ffc107;
    }
    
    &.symbol-fund {
      background-color: #607d8b;
    }
    
    &.symbol-default {
      background-color: #607d8b;
    }
  }
  
  .symbol-details {
    display: flex;
    flex-direction: column;
  }
  
  .symbol-title {
    font-size: 16px;
    font-weight: 500;
  }
  
  .symbol-price {
    font-size: 14px;
    margin-top: 4px;
    
    .price-change {
      margin-left: 8px;
      font-size: 13px;
    }
    
    &.positive {
      color: var(--positiveColor);
    }
    
    &.negative {
      color: var(--negativeColor);
    }
  }
  
  .chart-controls {
    display: flex;
    gap: 8px;
  }
  
  .chart-control-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background-color: var(--cardBg);
    border: 1px solid var(--borderColor);
    border-radius: 4px;
    color: var(--textColor);
    font-size: 13px;
    cursor: pointer;
    
    &:hover {
      background-color: var(--hover-bg);
    }
    
    svg {
      width: 16px;
      height: 16px;
    }
  }
  </style>
  