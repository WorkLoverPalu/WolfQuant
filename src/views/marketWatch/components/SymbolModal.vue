<template>
    <div class="modal-overlay" @click="closeModal">
      <div class="modal-container" @click.stop>
        <div class="modal-header">
          <h3>添加商品代码</h3>
          <button class="close-button" @click="closeModal">
            <XIcon />
          </button>
        </div>
        
        <div class="modal-search">
          <SearchIcon class="search-icon" />
          <input 
            type="text" 
            class="search-input" 
            placeholder="输入代码或名称搜索" 
            v-model="searchQuery"
            @input="searchSymbols"
          />
        </div>
        
        <div class="modal-tabs">
          <button 
            v-for="category in categories" 
            :key="category.id"
            class="modal-tab"
            :class="{ active: modalActiveCategory === category.id }"
            @click="setModalActiveCategory(category.id)"
          >
            {{ category.name }}
          </button>
        </div>
        
        <div class="modal-filters">
          <div class="filter-group">
            <button class="filter-button">
              所有国家/地区
              <ChevronDownIcon />
            </button>
            <button class="filter-button">
              所有类型
              <ChevronDownIcon />
            </button>
          </div>
        </div>
        
        <div class="modal-results">
          <div 
            v-for="(result, index) in searchResults" 
            :key="index"
            class="result-item"
          >
            <div class="result-info">
              <div class="result-icon" :class="getSymbolClass(result.symbol)">
                {{ getSymbolIcon(result.symbol) }}
              </div>
              <div class="result-details">
                <div class="result-code">
                  {{ result.symbol }}
                  <span class="result-highlight">{{ result.name }}</span>
                </div>
                <div class="result-meta">{{ result.meta }}</div>
              </div>
            </div>
            <button class="add-to-watchlist-button" @click="addSymbolToWatchlist(result)">
              添加到自选表
            </button>
          </div>
          
          <div v-if="searchResults.length === 0" class="empty-results">
            没有找到匹配的结果
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, inject, onMounted } from 'vue';
  import { 
    XIcon, 
    SearchIcon,
    ChevronDownIcon
  } from 'lucide-vue-next';
  
  // 注入全局状态
  const showAddSymbolModal = inject('showAddSymbolModal');
  const activeCategory = inject('activeCategory');
  const groups = inject('groups');
  
  // 分类数据
  const categories = [
    { id: 'fund', name: '基金' },
    { id: 'stock', name: '股票' },
    { id: 'gold', name: '黄金' },
    { id: 'crypto', name: '数字货币' }
  ];
  
  // 搜索相关状态
  const searchQuery = ref('');
  const searchResults = ref([]);
  const modalActiveCategory = ref(activeCategory.value);
  const currentTargetGroup = ref(null);
  
  // 关闭模态框
  const closeModal = () => {
    showAddSymbolModal.value = false;
    searchQuery.value = '';
    currentTargetGroup.value = null;
  };
  
  // 设置模态框分类
  const setModalActiveCategory = (category) => {
    modalActiveCategory.value = category;
    searchSymbols();
  };
  
  // 搜索商品
  const searchSymbols = () => {
    // 模拟搜索结果
    const mockResults = [
      { symbol: '518880', name: '黄金基金', meta: 'fund etf SSE', category: 'fund' },
      { symbol: '159934', name: '黄金ETF', meta: 'fund etf SZSE', category: 'fund' },
      { symbol: 'XAUUSD', name: '黄金/美元', meta: 'spot gold', category: 'gold' },
      { symbol: 'GC', name: '黄金期货', meta: 'futures COMEX', category: 'gold' }
    ];
    
    // 根据搜索词和当前分类过滤
    searchResults.value = mockResults.filter(item => {
      const matchesSearch = searchQuery.value === '' || 
        item.symbol.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        item.name.toLowerCase().includes(searchQuery.value.toLowerCase());
      
      const matchesCategory = modalActiveCategory.value === 'all' || 
        item.category === modalActiveCategory.value;
      
      return matchesSearch && matchesCategory;
    });
  };
  
  // 添加商品到自选表
  const addSymbolToWatchlist = (symbol) => {
    // 确定目标分组
    let targetGroup;
    
    if (currentTargetGroup.value) {
      targetGroup = groups.value.find(group => group.id === currentTargetGroup.value);
    } else {
      targetGroup = groups.value.find(group => group.category === symbol.category);
    }
    
    if (targetGroup) {
      // 检查是否已存在
      const exists = targetGroup.items.some(item => item.symbol === symbol.symbol);
      
      if (!exists) {
        // 添加到分组
        targetGroup.items.push({
          symbol: symbol.symbol,
          name: symbol.name,
          price: '0.00',
          unit: 'USD',
          change: '0.00',
          changePercent: '0.00%',
          volume: '—',
          turnover: '—'
        });
      }
    }
    
    closeModal();
  };
  
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
      '518880': 'symbol-fund',
      '159934': 'symbol-fund',
      'GC': 'symbol-gold'
    };
    
    return symbolMap[symbol] || 'symbol-default';
  };
  
  // 初始化
  onMounted(() => {
    searchSymbols();
  });
  </script>
  
  <style lang="scss" scoped>
  /* 弹窗样式 */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }
  
  .modal-container {
    width: 90%;
    max-width: 600px;
    max-height: 80vh;
    background-color: var(--modal-bg);
    border-radius: 8px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    border-bottom: 1px solid var(--border-color);
    
    h3 {
      margin: 0;
      font-size: 18px;
      font-weight: 500;
    }
    
    .close-button {
      width: 32px;
      height: 32px;
      border-radius: 4px;
      display: flex;
      align-items: center;
      justify-content: center;
      background: transparent;
      border: none;
      color: var(--text-secondary);
      cursor: pointer;
      
      &:hover {
        background-color: var(--hover-bg);
        color: var(--text-color);
      }
      
      svg {
        width: 20px;
        height: 20px;
      }
    }
  }
  
  .modal-search {
    position: relative;
    padding: 16px;
    border-bottom: 1px solid var(--border-color);
  }
  
  .search-icon {
    position: absolute;
    left: 28px;
    top: 50%;
    transform: translateY(-50%);
    width: 20px;
    height: 20px;
    color: var(--text-secondary);
  }
  
  .search-input {
    width: 100%;
    height: 40px;
    padding: 0 16px 0 40px;
    background-color: var(--input-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-color);
    font-size: 14px;
    
    &:focus {
      outline: none;
      border-color: var(--accent-color);
    }
    
    &::placeholder {
      color: var(--text-secondary);
    }
  }
  
  .modal-tabs {
    display: flex;
    overflow-x: auto;
    border-bottom: 1px solid var(--border-color);
    
    &::-webkit-scrollbar {
      height: 0;
    }
  }
  
  .modal-tab {
    padding: 12px 16px;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 14px;
    cursor: pointer;
    white-space: nowrap;
    
    &:hover {
      color: var(--text-color);
    }
    
    &.active {
      color: var(--accent-color);
      position: relative;
      
      &::after {
        content: '';
        position: absolute;
        bottom: -1px;
        left: 0;
        width: 100%;
        height: 2px;
        background-color: var(--accent-color);
      }
    }
  }
  
  .modal-filters {
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
  }
  
  .filter-group {
    display: flex;
    gap: 8px;
    overflow-x: auto;
    
    &::-webkit-scrollbar {
      height: 0;
    }
  }
  
  .filter-button {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 6px 12px;
    background-color: var(--input-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-color);
    font-size: 12px;
    cursor: pointer;
    white-space: nowrap;
    
    &:hover {
      background-color: var(--hover-bg);
    }
    
    svg {
      width: 14px;
      height: 14px;
    }
  }
  
  .modal-results {
    flex: 1;
    overflow-y: auto;
    padding: 0;
    
    &::-webkit-scrollbar {
      width: 4px;
    }
    
    &::-webkit-scrollbar-track {
      background: transparent;
    }
    
    &::-webkit-scrollbar-thumb {
      background: var(--scrollbar-thumb);
      border-radius: 2px;
    }
  }
  
  .result-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
    
    &:last-child {
      border-bottom: none;
    }
    
    &:hover {
      background-color: var(--hover-bg);
    }
  }
  
  .result-info {
    display: flex;
    align-items: center;
  }
  
  .result-icon {
    width: 32px;
    height: 32px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 12px;
    font-size: 14px;
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
  
  .result-details {
    display: flex;
    flex-direction: column;
  }
  
  .result-code {
    font-size: 14px;
    font-weight: 500;
    
    .result-highlight {
      margin-left: 8px;
      color: var(--text-secondary);
    }
  }
  
  .result-meta {
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 2px;
  }
  
  .add-to-watchlist-button {
    padding: 6px 12px;
    background-color: var(--button-bg);
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 12px;
    cursor: pointer;
    
    &:hover {
      background-color: var(--button-hover-bg);
    }
  }
  
  .empty-results {
    padding: 32px 16px;
    text-align: center;
    color: var(--text-secondary);
  }
  </style>
  