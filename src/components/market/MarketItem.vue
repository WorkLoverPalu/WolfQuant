<template>
    <div class="market-item">
      <div class="item-symbol">
        <div class="symbol-icon" :class="getSymbolClass()">{{ getSymbolIcon() }}</div>
        <div class="symbol-info">
          <div class="symbol-name">{{ symbol }}</div>
          <div class="symbol-fullname">{{ name }}</div>
        </div>
      </div>
      <div class="item-price">{{ price }}</div>
      <div class="item-change" :class="getChangeClass()">{{ change }}</div>
      <div class="item-change-percent" :class="getChangeClass()">{{ changePercent }}</div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { defineProps } from 'vue';
  
  const props = defineProps({
    symbol: {
      type: String,
      required: true
    },
    name: {
      type: String,
      required: true
    },
    price: {
      type: String,
      required: true
    },
    change: {
      type: String,
      required: true
    },
    changePercent: {
      type: String,
      required: true
    }
  });
  
  // 获取符号图标
  const getSymbolIcon = () => {
    return '';
  };
  
  // 获取符号类名
  const getSymbolClass = () => {
    const symbolMap: Record<string, string> = {
      'SPX': 'symbol-spx',
      'NDQ': 'symbol-ndq',
      'DJI': 'symbol-dji',
      'VIX': 'symbol-vix',
      'DXY': 'symbol-dxy',
      'AAPL': 'symbol-aapl',
      'TSLA': 'symbol-tsla',
      'NFLX': 'symbol-nflx',
      'USOIL': 'symbol-usoil',
      'GOLD': 'symbol-gold'
    };
    
    return symbolMap[props.symbol] || 'symbol-default';
  };
  
  // 获取涨跌类名
  const getChangeClass = () => {
    if (props.change.startsWith('-')) {
      return 'negative';
    } else if (props.change.startsWith('+') || parseFloat(props.change) > 0) {
      return 'positive';
    }
    return '';
  };
  </script>
  
  <style lang="scss" scoped>
  .market-item {
    display: flex;
    align-items: center;
    padding: 8px 16px;
    cursor: pointer;
    
    &:hover {
      background-color: rgba(255, 255, 255, 0.03);
    }
  }
  
  .item-symbol {
    display: flex;
    align-items: center;
    flex: 1;
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
    
    &.symbol-aapl {
      background-color: #607d8b;
    }
    
    &.symbol-tsla {
      background-color: #f44336;
    }
    
    &.symbol-nflx {
      background-color: #e91e63;
    }
    
    &.symbol-usoil {
      background-color: #795548;
    }
    
    &.symbol-gold {
      background-color: #ffc107;
    }
    
    &.symbol-default {
      background-color: #607d8b;
    }
  }
  
  .symbol-info {
    display: flex;
    flex-direction: column;
  }
  
  .symbol-name {
    font-size: 14px;
    color: #d1d4dc;
  }
  
  .symbol-fullname {
    font-size: 10px;
    color: #787b86;
  }
  
  .item-price, .item-change, .item-change-percent {
    width: 80px;
    text-align: right;
    font-size: 14px;
    color: #d1d4dc;
  }
  
  .item-change, .item-change-percent {
    &.positive {
      color: #26a69a;
    }
    
    &.negative {
      color: #ef5350;
    }
  }
  </style>