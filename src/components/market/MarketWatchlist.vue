<template>
    <div class="market-watchlist">
      <div class="watchlist-header">
        <div class="watchlist-title">
          <span>Watchlist</span>
          <button class="dropdown-button">
            <i class="icon-chevron-down"></i>
          </button>
        </div>
        <div class="watchlist-actions">
          <button class="action-button">
            <i class="icon-plus"></i>
          </button>
          <button class="action-button">
            <i class="icon-clock"></i>
          </button>
          <button class="action-button">
            <i class="icon-more"></i>
          </button>
        </div>
      </div>
      
      <div class="watchlist-columns">
        <div class="column-name">商品代码</div>
        <div class="column-price">最新价</div>
        <div class="column-change">涨跌</div>
        <div class="column-change-percent">涨跌%</div>
      </div>
      
      <div class="watchlist-category">
        <div class="category-header">
          <div class="category-name">
            <i class="icon-chevron-down"></i>
            INDICES
          </div>
        </div>
        
        <div class="watchlist-items">
          <MarketItem 
            v-for="item in indices" 
            :key="item.symbol"
            :symbol="item.symbol"
            :name="item.name"
            :price="item.price"
            :change="item.change"
            :changePercent="item.changePercent"
          />
        </div>
      </div>
      
      <div class="watchlist-category">
        <div class="category-header">
          <div class="category-name">
            <i class="icon-chevron-down"></i>
            STOCKS
          </div>
        </div>
        
        <div class="watchlist-items">
          <MarketItem 
            v-for="item in stocks" 
            :key="item.symbol"
            :symbol="item.symbol"
            :name="item.name"
            :price="item.price"
            :change="item.change"
            :changePercent="item.changePercent"
          />
        </div>
      </div>
      
      <div class="watchlist-category">
        <div class="category-header">
          <div class="category-name">
            <i class="icon-chevron-down"></i>
            FUTURES
          </div>
        </div>
        
        <div class="watchlist-items">
          <MarketItem 
            v-for="item in futures" 
            :key="item.symbol"
            :symbol="item.symbol"
            :name="item.name"
            :price="item.price"
            :change="item.change"
            :changePercent="item.changePercent"
          />
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref } from 'vue';
  import MarketItem from './MarketItem.vue';
  
  // 模拟数据
  const indices = ref([
    { symbol: 'SPX', name: 'S&P 500', price: '5,663.49', change: '-0.46', changePercent: '-0.01%' },
    { symbol: 'NDQ', name: 'NASDAQ', price: '20,076.90', change: '13.33', changePercent: '0.07%' },
    { symbol: 'DJI', name: 'Dow Jones', price: '41,271.95', change: '-96.50', changePercent: '-0.23%' },
    { symbol: 'VIX', name: 'Volatility Index', price: '22.45', change: '-0.03', changePercent: '-0.13%' },
    { symbol: 'DXY', name: 'US Dollar Index', price: '100.265', change: '-0.370', changePercent: '-0.37%' }
  ]);
  
  const stocks = ref([
    { symbol: 'AAPL', name: 'Apple Inc', price: '198.64', change: '1.15', changePercent: '0.58%' },
    { symbol: 'TSLA', name: 'Tesla Inc', price: '301.76', change: '16.94', changePercent: '5.95%' },
    { symbol: 'NFLX', name: 'Netflix Inc', price: '1,140.86', change: '-3.57', changePercent: '-0.31%' }
  ]);
  
  const futures = ref([
    { symbol: 'USOIL', name: 'Crude Oil', price: '60.58', change: '0.37', changePercent: '0.61%' },
    { symbol: 'GOLD', name: 'Gold', price: '3,340.905', change: '38.438', changePercent: '1.10%' }
  ]);
  </script>
  
  <style lang="scss" scoped>
  .market-watchlist {
    background-color: #1e222d;
    border-radius: 8px;
    overflow: hidden;
  }
  
  .watchlist-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid #2a2e39;
  }
  
  .watchlist-title {
    display: flex;
    align-items: center;
    font-size: 16px;
    font-weight: 500;
    
    .dropdown-button {
      background: transparent;
      border: none;
      color: #787b86;
      margin-left: 4px;
      cursor: pointer;
      
      &:hover {
        color: #d1d4dc;
      }
    }
  }
  
  .watchlist-actions {
    display: flex;
    gap: 8px;
  }
  
  .action-button {
    background: transparent;
    border: none;
    color: #787b86;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    border-radius: 4px;
    
    &:hover {
      background-color: rgba(255, 255, 255, 0.05);
      color: #d1d4dc;
    }
  }
  
  .watchlist-columns {
    display: flex;
    padding: 8px 16px;
    font-size: 12px;
    color: #787b86;
    border-bottom: 1px solid #2a2e39;
  }
  
  .column-name {
    flex: 1;
  }
  
  .column-price, .column-change, .column-change-percent {
    width: 80px;
    text-align: right;
  }
  
  .watchlist-category {
    border-bottom: 1px solid #2a2e39;
    
    &:last-child {
      border-bottom: none;
    }
  }
  
  .category-header {
    padding: 8px 16px;
    background-color: rgba(0, 0, 0, 0.1);
  }
  
  .category-name {
    display: flex;
    align-items: center;
    font-size: 12px;
    color: #787b86;
    
    i {
      margin-right: 4px;
      font-size: 10px;
    }
  }
  
  .watchlist-items {
    max-height: 300px;
    overflow-y: auto;
    
    &::-webkit-scrollbar {
      width: 4px;
    }
    
    &::-webkit-scrollbar-track {
      background: transparent;
    }
    
    &::-webkit-scrollbar-thumb {
      background: #2a2e39;
      border-radius: 2px;
    }
  }
  </style>