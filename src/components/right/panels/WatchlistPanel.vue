<template>
    <div class="watchlist-panel">
      <div class="panel-toolbar">
        <div class="search-container">
          <input 
            type="text" 
            class="search-input" 
            placeholder="搜索自选股..." 
            v-model="searchQuery"
          />
          <SearchIcon class="search-icon" />
        </div>
        <button class="add-button" @click="openAddSymbolModal">
          <PlusIcon />
        </button>
      </div>
      
      <div class="watchlist-groups">
        <div 
          v-for="(group, groupIndex) in filteredGroups" 
          :key="groupIndex"
          class="watchlist-group"
        >
          <div class="group-header" @click="toggleGroup(group.id)">
            <div class="group-title">
              <component :is="expandedGroups.includes(group.id) ? ChevronDownIcon : ChevronRightIcon" class="chevron-icon" />
              {{ group.name }}
            </div>
            <div class="group-actions">
              <button class="action-button">
                <MoreVerticalIcon />
              </button>
            </div>
          </div>
          
          <div v-if="expandedGroups.includes(group.id)" class="group-content">
            <div 
              v-for="(item, itemIndex) in group.items" 
              :key="itemIndex"
              class="watchlist-item"
              @click="openSymbolTab(item)"
            >
              <div class="item-info">
                <div class="symbol-icon" :class="getSymbolClass(item.symbol)">
                  {{ getSymbolIcon(item.symbol) }}
                </div>
                <div class="symbol-details">
                  <div class="symbol-name">{{ item.symbol }}</div>
                  <div class="symbol-fullname">{{ item.name }}</div>
                </div>
              </div>
              <div class="item-price">
                <div class="price-value">{{ item.price }}</div>
                <div 
                  class="price-change" 
                  :class="{ 
                    'positive': parseFloat(item.change) > 0, 
                    'negative': parseFloat(item.change) < 0 
                  }"
                >
                  {{ item.change }} ({{ item.changePercent }})
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 添加股票弹窗 -->
      <div v-if="showAddSymbolModal" class="modal">
        <div class="modal-content">
          <div class="modal-header">
            <h3>添加自选股</h3>
            <button class="close-button" @click="showAddSymbolModal = false">
              <XIcon />
            </button>
          </div>
          <div class="modal-body">
            <div class="search-container">
              <input 
                type="text" 
                class="search-input" 
                placeholder="输入股票代码或名称..." 
                v-model="symbolSearchQuery"
              />
              <SearchIcon class="search-icon" />
            </div>
            
            <div class="search-results">
              <div 
                v-for="(result, index) in searchResults" 
                :key="index"
                class="search-result"
                @click="addToWatchlist(result)"
              >
                <div class="result-info">
                  <div class="symbol-icon" :class="getSymbolClass(result.symbol)">
                    {{ getSymbolIcon(result.symbol) }}
                  </div>
                  <div class="symbol-details">
                    <div class="symbol-name">{{ result.symbol }}</div>
                    <div class="symbol-fullname">{{ result.name }}</div>
                  </div>
                </div>
                <button class="add-to-watchlist-button">
                  <PlusIcon />
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed } from 'vue';
  import { 
    SearchIcon, 
    PlusIcon, 
    ChevronDownIcon, 
    ChevronRightIcon, 
    MoreVerticalIcon,
    XIcon
  } from 'lucide-vue-next';
  
  // 定义事件
  const emit = defineEmits<{
    (e: 'close'): void;
    (e: 'open-tab', tabData: any): void;
  }>();
  
  // 搜索查询
  const searchQuery = ref('');
  
  // 展开的分组
  const expandedGroups = ref(['indices', 'stocks']);
  
  // 添加股票弹窗
  const showAddSymbolModal = ref(false);
  const symbolSearchQuery = ref('');
  
  // 自选股分组数据
  const watchlistGroups = ref([
    {
      id: 'indices',
      name: '指数',
      items: [
        { symbol: 'SPX', name: 'S&P 500', price: '5,659.90', change: '-4.05', changePercent: '-0.07%' },
        { symbol: 'NDQ', name: 'NASDAQ', price: '20,061.45', change: '-2.12', changePercent: '-0.01%' },
        { symbol: 'DJI', name: 'Dow Jones', price: '41,249.38', change: '-119.07', changePercent: '-0.29%' }
      ]
    },
    {
      id: 'stocks',
      name: '股票',
      items: [
        { symbol: 'AAPL', name: 'Apple Inc', price: '198.53', change: '+1.04', changePercent: '+0.53%' },
        { symbol: 'MSFT', name: 'Microsoft Corp', price: '420.45', change: '+2.30', changePercent: '+0.55%' },
        { symbol: 'GOOGL', name: 'Alphabet Inc', price: '175.98', change: '-0.87', changePercent: '-0.49%' }
      ]
    },
    {
      id: 'crypto',
      name: '加密货币',
      items: [
        { symbol: 'BTCUSD', name: 'Bitcoin', price: '67,890.50', change: '+1,234.56', changePercent: '+1.85%' },
        { symbol: 'ETHUSD', name: 'Ethereum', price: '3,456.78', change: '+98.76', changePercent: '+2.94%' }
      ]
    }
  ]);
  
  // 搜索结果
  const searchResults = computed(() => {
    if (!symbolSearchQuery.value) {
      return [
        { symbol: 'AAPL', name: 'Apple Inc' },
        { symbol: 'MSFT', name: 'Microsoft Corp' },
        { symbol: 'GOOGL', name: 'Alphabet Inc' },
        { symbol: 'AMZN', name: 'Amazon.com Inc' },
        { symbol: 'TSLA', name: 'Tesla Inc' }
      ];
    }
    
    const query = symbolSearchQuery.value.toLowerCase();
    return [
      { symbol: 'AAPL', name: 'Apple Inc' },
      { symbol: 'MSFT', name: 'Microsoft Corp' },
      { symbol: 'GOOGL', name: 'Alphabet Inc' },
      { symbol: 'AMZN', name: 'Amazon.com Inc' },
      { symbol: 'TSLA', name: 'Tesla Inc' }
    ].filter(item => 
      item.symbol.toLowerCase().includes(query) || 
      item.name.toLowerCase().includes(query)
    );
  });
  
  // 根据搜索过滤分组
  const filteredGroups = computed(() => {
    if (!searchQuery.value) {
      return watchlistGroups.value;
    }
    
    const query = searchQuery.value.toLowerCase();
    return watchlistGroups.value.map(group => {
      return {
        ...group,
        items: group.items.filter(item => 
          item.symbol.toLowerCase().includes(query) || 
          item.name.toLowerCase().includes(query)
        )
      };
    }).filter(group => group.items.length > 0);
  });
  
  // 切换分组展开/折叠
  const toggleGroup = (groupId: string) => {
    const index = expandedGroups.value.indexOf(groupId);
    if (index === -1) {
      expandedGroups.value.push(groupId);
    } else {
      expandedGroups.value.splice(index, 1);
    }
  };
  
  // 打开添加股票弹窗
  const openAddSymbolModal = () => {
    showAddSymbolModal.value = true;
  };
  
  // 添加到自选股
  const addToWatchlist = (symbol: any) => {
    // 添加到股票分组
    watchlistGroups.value[1].items.push({
      symbol: symbol.symbol,
      name: symbol.name,
      price: '0.00',
      change: '0.00',
      changePercent: '0.00%'
    });
    
    // 确保分组是展开的
    if (!expandedGroups.value.includes('stocks')) {
      expandedGroups.value.push('stocks');
    }
    
    // 关闭弹窗
    showAddSymbolModal.value = false;
  };
  
  // 获取符号图标
  const getSymbolIcon = (symbol: string) => {
    return symbol.charAt(0);
  };
  
  // 获取符号类名
  const getSymbolClass = (symbol: string) => {
    const symbolMap: Record<string, string> = {
      'SPX': 'symbol-spx',
      'NDQ': 'symbol-ndq',
      'DJI': 'symbol-dji',
      'AAPL': 'symbol-aapl',
      'MSFT': 'symbol-msft',
      'GOOGL': 'symbol-googl',
      'AMZN': 'symbol-amzn',
      'TSLA': 'symbol-tsla',
      'BTCUSD': 'symbol-btc',
      'ETHUSD': 'symbol-eth'
    };
    
    return symbolMap[symbol] || 'symbol-default';
  };
  
  // 打开股票标签页
  const openSymbolTab = (item: any) => {
    emit('open-tab', {
      id: `symbol-${item.symbol}`,
      title: item.symbol,
      component: 'SymbolDetail',
      closable: true,
      props: {
        symbol: item.symbol,
        name: item.name,
        price: item.price,
        change: item.change,
        changePercent: item.changePercent
      }
    });
  };
  </script>
  
  <style lang="scss" scoped>
  .watchlist-panel {
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  
  .panel-toolbar {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--panel-border, #333333);
  }
  
  .search-container {
    flex: 1;
    position: relative;
  }
  
  .search-input {
    width: 100%;
    height: 36px;
    padding: 0 36px 0 12px;
    background-color: var(--input-bg, #2c2c2c);
    border: 1px solid var(--panel-border, #333333);
    border-radius: 4px;
    color: var(--panel-text, #ffffff);
    font-size: 14px;
    
    &:focus {
      outline: none;
      border-color: var(--icon-active-color, #2563eb);
    }
    
    &::placeholder {
      color: var(--panel-secondary-text, #a0a0a0);
    }
  }
  
  .search-icon {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    width: 16px;
    height: 16px;
    color: var(--panel-secondary-text, #a0a0a0);
    pointer-events: none;
  }
  
  .add-button {
    width: 36px;
    height: 36px;
    margin-left: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: transparent;
    border: none;
    border-radius: 4px;
    color: var(--icon-color, #a0a0a0);
    cursor: pointer;
    
    &:hover {
      background-color: rgba(128, 128, 128, 0.1);
      color: var(--icon-active-color, #2563eb);
    }
    
    svg {
      width: 18px;
      height: 18px;
    }
  }
  
  .watchlist-groups {
    flex: 1;
    overflow-y: auto;
    
    &::-webkit-scrollbar {
      width: 6px;
    }
    
    &::-webkit-scrollbar-track {
      background: transparent;
    }
    
    &::-webkit-scrollbar-thumb {
      background-color: rgba(128, 128, 128, 0.3);
      border-radius: 3px;
    }
  }
  
  .watchlist-group {
    border-bottom: 1px solid var(--panel-border, #333333);
    
    &:last-child {
      border-bottom: none;
    }
  }
  
  .group-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    cursor: pointer;
    
    &:hover {
      background-color: rgba(128, 128, 128, 0.05);
    }
  }
  
  .group-title {
    display: flex;
    align-items: center;
    font-weight: 500;
    color: var(--panel-text, #ffffff);
    
    .chevron-icon {
      width: 16px;
      height: 16px;
      margin-right: 8px;
    }
  }
  
  .group-actions {
    display: flex;
  }
  
  .action-button {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: transparent;
    border: none;
    border-radius: 4px;
    color: var(--icon-color, #a0a0a0);
    cursor: pointer;
    
    &:hover {
      background-color: rgba(128, 128, 128, 0.1);
      color: var(--panel-text, #ffffff);
    }
    
    svg {
      width: 16px;
      height: 16px;
    }
  }
  
  .watchlist-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    cursor: pointer;
    
    &:hover {
      background-color: rgba(128, 128, 128, 0.05);
    }
  }
  
  .item-info {
    display: flex;
    align-items: center;
  }
  
  .symbol-icon {
    width: 32px;
    height: 32px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 12px;
    font-size: 14px;
    font-weight: 500;
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
    
    &.symbol-aapl {
      background-color: #607d8b;
    }
    
    &.symbol-msft {
      background-color: #3f51b5;
    }
    
    &.symbol-googl {
      background-color: #ff9800;
    }
    
    &.symbol-amzn {
      background-color: #795548;
    }
    
    &.symbol-tsla {
      background-color: #f44336;
    }
    
    &.symbol-btc {
      background-color: #ff9800;
    }
    
    &.symbol-eth {
      background-color: #9c27b0;
    }
    
    &.symbol-default {
      background-color: #607d8b;
    }
  }
  
  .symbol-details {
    display: flex;
    flex-direction: column;
  }
  
  .symbol-name {
    font-size: 14px;
    font-weight: 500;
    color: var(--panel-text, #ffffff);
  }
  
  .symbol-fullname {
    font-size: 12px;
    color: var(--panel-secondary-text, #a0a0a0);
  }
  
  .item-price {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }
  
  .price-value {
    font-size: 14px;
    font-weight: 500;
    color: var(--panel-text, #ffffff);
  }
  
  .price-change {
    font-size: 12px;
    
    &.positive {
      color: #4caf50;
    }
    
    &.negative {
      color: #f44336;
    }
  }
  
  /* 弹窗样式 */
  .modal {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1100;
  }
  
  .modal-content {
    width: 90%;
    max-width: 480px;
    max-height: 80vh;
    background-color: var(--panel-bg, #1e1e1e);
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
    border-bottom: 1px solid var(--panel-border, #333333);
    
    h3 {
      margin: 0;
      font-size: 18px;
      font-weight: 500;
      color: var(--panel-text, #ffffff);
    }
    
    .close-button {
      width: 28px;
      height: 28px;
      display: flex;
      align-items: center;
      justify-content: center;
      background-color: transparent;
      border: none;
      border-radius: 4px;
      color: var(--icon-color, #a0a0a0);
      cursor: pointer;
      
      &:hover {
        background-color: rgba(128, 128, 128, 0.1);
        color: var(--panel-text, #ffffff);
      }
      
      svg {
        width: 18px;
        height: 18px;
      }
    }
  }
  
  .modal-body {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    
    .search-container {
      margin-bottom: 16px;
    }
  }
  
  .search-results {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  
  .search-result {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px;
    border-radius: 4px;
    background-color: rgba(128, 128, 128, 0.05);
    cursor: pointer;
    
    &:hover {
      background-color: rgba(128, 128, 128, 0.1);
    }
  }
  
  .result-info {
    display: flex;
    align-items: center;
  }
  
  .add-to-watchlist-button {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--icon-active-color, #2563eb);
    border: none;
    border-radius: 4px;
    color: white;
    cursor: pointer;
    
    &:hover {
      background-color: var(--button-primary-hover, #3b82f6);
    }
    
    svg {
      width: 16px;
      height: 16px;
    }
  }
  </style>