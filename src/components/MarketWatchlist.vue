<template>
    <div class="market-watchlist" :class="{ 'light-theme': !isDarkTheme }">
      <!-- 顶部导航栏 -->
      <div class="watchlist-header">
        <div class="category-tabs">
          <button 
            v-for="category in categories" 
            :key="category.id"
            class="category-tab"
            :class="{ active: activeCategory === category.id }"
            @click="setActiveCategory(category.id)"
          >
            {{ category.name }}
          </button>
        </div>
        
        <div class="header-actions">
          <button class="theme-toggle" @click="toggleTheme">
            <SunIcon v-if="isDarkTheme" />
            <MoonIcon v-else />
          </button>
          <button class="add-button" @click="openAddSymbolModal">
            <PlusIcon />
          </button>
        </div>
      </div>
      
      <!-- 分组列表 -->
      <div class="watchlist-groups">
        <div 
          v-for="(group, index) in filteredGroups" 
          :key="group.id"
          class="watchlist-group"
          :class="{ 'is-dragging': draggedGroup === group.id }"
          draggable="true"
          @dragstart="handleDragStart($event, group.id)"
          @dragover.prevent="handleDragOver($event, group.id)"
          @dragend="handleDragEnd"
          @drop="handleDrop($event, group.id)"
        >
          <div class="group-header">
            <div class="group-title" @click="toggleGroup(group.id)">
              <ChevronDownIcon v-if="expandedGroups.includes(group.id)" />
              <ChevronRightIcon v-else />
              {{ group.name }}
            </div>
            <div class="group-actions">
              <button class="action-button" @click="editGroup(group)">
                <EditIcon />
              </button>
              <button class="action-button" @click="deleteGroup(group.id)">
                <TrashIcon />
              </button>
            </div>
          </div>
          
          <div v-if="expandedGroups.includes(group.id)" class="group-content">
            <!-- 列表标题 -->
            <div class="list-header">
              <div class="column symbol-column">商品代码</div>
              <div class="column price-column">最新价</div>
              <div class="column change-column">涨跌</div>
              <div class="column change-percent-column">涨跌%</div>
              <div class="column volume-column">成交量</div>
              <div class="column turnover-column">成交额</div>
            </div>
            
            <!-- 列表内容 -->
            <div class="list-content">
              <div 
                v-for="item in group.items" 
                :key="item.symbol"
                class="list-item"
              >
                <div class="column symbol-column">
                  <div class="symbol-info">
                    <div class="symbol-icon" :class="getSymbolClass(item.symbol)">
                      {{ getSymbolIcon(item.symbol) }}
                    </div>
                    <div class="symbol-details">
                      <div class="symbol-code">{{ item.symbol }}</div>
                      <div class="symbol-name">{{ item.name }}</div>
                    </div>
                  </div>
                </div>
                <div class="column price-column">
                  {{ item.price }}
                  <span class="unit">{{ item.unit }}</span>
                </div>
                <div 
                  class="column change-column"
                  :class="getChangeClass(item.change)"
                >
                  {{ item.change }}
                </div>
                <div 
                  class="column change-percent-column"
                  :class="getChangeClass(item.changePercent)"
                >
                  {{ item.changePercent }}
                </div>
                <div class="column volume-column">{{ item.volume || '—' }}</div>
                <div class="column turnover-column">{{ item.turnover || '—' }}</div>
              </div>
              
              <!-- 空状态 -->
              <div v-if="group.items.length === 0" class="empty-state">
                <div class="empty-message">该分组暂无数据</div>
                <button class="add-item-button" @click="openAddSymbolModal">
                  <PlusIcon />
                  添加商品
                </button>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 添加分组按钮 -->
        <div class="add-group-container">
          <button class="add-group-button" @click="openAddGroupModal">
            <PlusIcon />
            添加分组
          </button>
        </div>
      </div>
      
      <!-- 添加商品弹窗 -->
      <div v-if="showAddSymbolModal" class="modal-overlay" @click="closeAddSymbolModal">
        <div class="modal-container" @click.stop>
          <div class="modal-header">
            <h3>添加商品代码</h3>
            <button class="close-button" @click="closeAddSymbolModal">
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
              @click="modalActiveCategory = category.id"
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
      
      <!-- 添加/编辑分组弹窗 -->
      <div v-if="showGroupModal" class="modal-overlay" @click="closeGroupModal">
        <div class="modal-container group-modal" @click.stop>
          <div class="modal-header">
            <h3>{{ editingGroup ? '编辑分组' : '添加分组' }}</h3>
            <button class="close-button" @click="closeGroupModal">
              <XIcon />
            </button>
          </div>
          
          <div class="group-form">
            <div class="form-group">
              <label for="groupName">分组名称</label>
              <input 
                type="text" 
                id="groupName" 
                class="form-input" 
                v-model="groupForm.name"
                placeholder="请输入分组名称"
              />
            </div>
            
            <div class="form-group">
              <label for="groupCategory">所属分类</label>
              <select id="groupCategory" class="form-select" v-model="groupForm.category">
                <option v-for="category in categories" :key="category.id" :value="category.id">
                  {{ category.name }}
                </option>
              </select>
            </div>
          </div>
          
          <div class="modal-footer">
            <button class="cancel-button" @click="closeGroupModal">取消</button>
            <button class="confirm-button" @click="saveGroup">确定</button>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed, onMounted } from 'vue';
  import { 
    ChevronDownIcon, 
    ChevronRightIcon, 
    PlusIcon, 
    EditIcon, 
    TrashIcon, 
    XIcon, 
    SearchIcon,
    SunIcon,
    MoonIcon
  } from 'lucide-vue-next';
  
  // 主题状态
  const isDarkTheme = ref(true);
  
  // 切换主题
  const toggleTheme = () => {
    isDarkTheme.value = !isDarkTheme.value;
  };
  
  // 分类数据
  const categories = [
    { id: 'fund', name: '基金' },
    { id: 'stock', name: '股票' },
    { id: 'gold', name: '黄金' },
    { id: 'crypto', name: '数字货币' }
  ];
  
  // 当前激活的分类
  const activeCategory = ref('fund');
  
  // 设置激活分类
  const setActiveCategory = (categoryId: string) => {
    activeCategory.value = categoryId;
  };
  
  // 分组数据
  const groups = ref([
    {
      id: 'indices',
      name: 'INDICES',
      category: 'stock',
      items: [
        { symbol: 'SPX', name: '标准普尔500指数', price: '5,659.90', unit: 'USD', change: '-4.05', changePercent: '-0.07%', volume: '2.39B', turnover: '2.91B' },
        { symbol: 'NDQ', name: 'US 100 Index', price: '20,061.45', unit: 'USD', change: '-2.12', changePercent: '-0.01%', volume: '—', turnover: '—' },
        { symbol: 'DJI', name: '道琼斯工业股票平均价格指数', price: '41,249.38', unit: 'USD', change: '-119.07', changePercent: '-0.29%', volume: '—', turnover: '—' },
        { symbol: 'VIX', name: '标普500波动率指数', price: '21.90', unit: 'POINT', change: '-0.58', changePercent: '-2.58%', volume: '—', turnover: '—' },
        { symbol: 'DXY', name: '美元指数', price: '100.424', unit: 'USD', change: '-0.212', changePercent: '-0.21%', volume: '—', turnover: '—' }
      ]
    },
    {
      id: 'crypto',
      name: '数字货币',
      category: 'crypto',
      items: [
        { symbol: 'BTCUSD', name: '比特币/美元', price: '67,890.50', unit: 'USD', change: '+1,234.56', changePercent: '+1.85%', volume: '32.5B', turnover: '45.7B' },
        { symbol: 'ETHUSD', name: '以太坊/美元', price: '3,456.78', unit: 'USD', change: '+98.76', changePercent: '+2.94%', volume: '15.2B', turnover: '22.3B' }
      ]
    },
    {
      id: 'gold',
      name: '黄金',
      category: 'gold',
      items: [
        { symbol: 'XAUUSD', name: '黄金/美元', price: '2,345.67', unit: 'USD', change: '+12.34', changePercent: '+0.53%', volume: '1.2B', turnover: '3.4B' }
      ]
    },
    {
      id: 'funds',
      name: '基金',
      category: 'fund',
      items: [
        { symbol: '518880', name: '黄金基金', price: '2.456', unit: 'CNY', change: '+0.023', changePercent: '+0.95%', volume: '123.4M', turnover: '345.6M' }
      ]
    }
  ]);
  
  // 根据当前分类过滤分组
  const filteredGroups = computed(() => {
    if (activeCategory.value === 'all') {
      return groups.value;
    }
    return groups.value.filter(group => group.category === activeCategory.value);
  });
  
  // 展开的分组
  const expandedGroups = ref(['indices', 'crypto', 'gold', 'funds']);
  
  // 切换分组展开/折叠
  const toggleGroup = (groupId: string) => {
    const index = expandedGroups.value.indexOf(groupId);
    if (index === -1) {
      expandedGroups.value.push(groupId);
    } else {
      expandedGroups.value.splice(index, 1);
    }
  };
  
  // 获取符号图标
  const getSymbolIcon = (symbol: string) => {
    const firstChar = symbol.charAt(0);
    return firstChar;
  };
  
  // 获取符号类名
  const getSymbolClass = (symbol: string) => {
    const symbolMap: Record<string, string> = {
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
  const getChangeClass = (change: string) => {
    if (change.startsWith('-')) {
      return 'negative';
    } else if (change.startsWith('+') || parseFloat(change.replace(',', '')) > 0) {
      return 'positive';
    }
    return '';
  };
  
  // 拖拽排序相关
  const draggedGroup = ref<string | null>(null);
  
  const handleDragStart = (event: DragEvent, groupId: string) => {
    draggedGroup.value = groupId;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
      event.dataTransfer.setData('text/plain', groupId);
    }
  };
  
  const handleDragOver = (event: DragEvent, groupId: string) => {
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = 'move';
    }
  };
  
  const handleDragEnd = () => {
    draggedGroup.value = null;
  };
  
  const handleDrop = (event: DragEvent, targetGroupId: string) => {
    if (!draggedGroup.value || draggedGroup.value === targetGroupId) return;
    
    const sourceIndex = groups.value.findIndex(g => g.id === draggedGroup.value);
    const targetIndex = groups.value.findIndex(g => g.id === targetGroupId);
    
    if (sourceIndex !== -1 && targetIndex !== -1) {
      const [movedGroup] = groups.value.splice(sourceIndex, 1);
      groups.value.splice(targetIndex, 0, movedGroup);
    }
    
    draggedGroup.value = null;
  };
  
  // 添加商品弹窗
  const showAddSymbolModal = ref(false);
  const searchQuery = ref('');
  const searchResults = ref<any[]>([]);
  const modalActiveCategory = ref('fund');
  
  const openAddSymbolModal = () => {
    showAddSymbolModal.value = true;
    modalActiveCategory.value = activeCategory.value;
    searchSymbols();
  };
  
  const closeAddSymbolModal = () => {
    showAddSymbolModal.value = false;
    searchQuery.value = '';
  };
  
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
  
  const addSymbolToWatchlist = (symbol: any) => {
    // 查找对应分类的分组
    const targetGroup = groups.value.find(group => group.category === symbol.category);
    
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
        
        // 确保分组是展开的
        if (!expandedGroups.value.includes(targetGroup.id)) {
          expandedGroups.value.push(targetGroup.id);
        }
      }
    }
    
    closeAddSymbolModal();
  };
  
  // 添加/编辑分组弹窗
  const showGroupModal = ref(false);
  const editingGroup = ref<any>(null);
  const groupForm = ref({
    name: '',
    category: 'fund'
  });
  
  const openAddGroupModal = () => {
    editingGroup.value = null;
    groupForm.value = {
      name: '',
      category: activeCategory.value
    };
    showGroupModal.value = true;
  };
  
  const editGroup = (group: any) => {
    editingGroup.value = group;
    groupForm.value = {
      name: group.name,
      category: group.category
    };
    showGroupModal.value = true;
  };
  
  const closeGroupModal = () => {
    showGroupModal.value = false;
  };
  
  const saveGroup = () => {
    if (!groupForm.value.name.trim()) {
      alert('请输入分组名称');
      return;
    }
    
    if (editingGroup.value) {
      // 更新现有分组
      const group = groups.value.find(g => g.id === editingGroup.value.id);
      if (group) {
        group.name = groupForm.value.name;
        group.category = groupForm.value.category;
      }
    } else {
      // 添加新分组
      const newGroupId = 'group_' + Date.now();
      groups.value.push({
        id: newGroupId,
        name: groupForm.value.name,
        category: groupForm.value.category,
        items: []
      });
      
      // 自动展开新分组
      expandedGroups.value.push(newGroupId);
    }
    
    closeGroupModal();
  };
  
  // 删除分组
  const deleteGroup = (groupId: string) => {
    if (confirm('确定要删除此分组吗？')) {
      const index = groups.value.findIndex(g => g.id === groupId);
      if (index !== -1) {
        groups.value.splice(index, 1);
        
        // 从展开列表中移除
        const expandedIndex = expandedGroups.value.indexOf(groupId);
        if (expandedIndex !== -1) {
          expandedGroups.value.splice(expandedIndex, 1);
        }
      }
    }
  };
  
  // 初始化
  onMounted(() => {
    // 可以在这里加载保存的数据
    console.log('市场行情列表组件已加载');
  });
  </script>
  
  <style lang="scss" scoped>
  /* 主题变量 */
  :root {
    --bg-color: #121212;
    --card-bg: #1e1e1e;
    --header-bg: #1a1a1a;
    --border-color: #333333;
    --text-color: #ffffff;
    --text-secondary: #a0a0a0;
    --accent-color: #2962ff;
    --positive-color: #26a69a;
    --negative-color: #ef5350;
    --hover-bg: rgba(255, 255, 255, 0.05);
    --active-bg: rgba(255, 255, 255, 0.1);
    --input-bg: #2c2c2c;
    --button-bg: #2962ff;
    --button-hover-bg: #1e53e4;
    --modal-bg: #1e1e1e;
    --scrollbar-thumb: #555555;
  }
  
  .light-theme {
    --bg-color: #f5f5f5;
    --card-bg: #ffffff;
    --header-bg: #ffffff;
    --border-color: #e0e0e0;
    --text-color: #333333;
    --text-secondary: #666666;
    --accent-color: #1a73e8;
    --positive-color: #4caf50;
    --negative-color: #f44336;
    --hover-bg: rgba(0, 0, 0, 0.03);
    --active-bg: rgba(0, 0, 0, 0.05);
    --input-bg: #f5f5f5;
    --button-bg: #1a73e8;
    --button-hover-bg: #1967d2;
    --modal-bg: #ffffff;
    --scrollbar-thumb: #cccccc;
  }
  
  /* 基础样式 */
  .market-watchlist {
    width: 100%;
    height: 100vh;
    background-color: var(--bg-color);
    color: var(--text-color);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  
  /* 顶部导航栏 */
  .watchlist-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 16px;
    height: 56px;
    background-color: var(--header-bg);
    border-bottom: 1px solid var(--border-color);
  }
  
  .category-tabs {
    display: flex;
    gap: 8px;
  }
  
  .category-tab {
    padding: 8px 16px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 14px;
    cursor: pointer;
    border-radius: 4px;
    
    &:hover {
      background-color: var(--hover-bg);
      color: var(--text-color);
    }
    
    &.active {
      background-color: var(--active-bg);
      color: var(--text-color);
      font-weight: 500;
    }
  }
  
  .header-actions {
    display: flex;
    gap: 8px;
  }
  
  .theme-toggle, .add-button {
    width: 36px;
    height: 36px;
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
  
  /* 分组列表 */
  .watchlist-groups {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    
    &::-webkit-scrollbar {
      width: 6px;
    }
    
    &::-webkit-scrollbar-track {
      background: transparent;
    }
    
    &::-webkit-scrollbar-thumb {
      background: var(--scrollbar-thumb);
      border-radius: 3px;
    }
  }
  
  .watchlist-group {
    margin-bottom: 16px;
    border-radius: 8px;
    background-color: var(--card-bg);
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    
    &.is-dragging {
      opacity: 0.7;
    }
  }
  
  .group-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background-color: var(--header-bg);
    border-bottom: 1px solid var(--border-color);
  }
  
  .group-title {
    display: flex;
    align-items: center;
    font-weight: 500;
    cursor: pointer;
    
    svg {
      width: 18px;
      height: 18px;
      margin-right: 8px;
    }
  }
  
  .group-actions {
    display: flex;
    gap: 8px;
  }
  
  .action-button {
    width: 28px;
    height: 28px;
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
      width: 16px;
      height: 16px;
    }
  }
  
  /* 列表标题 */
  .list-header {
    display: flex;
    padding: 8px 16px;
    border-bottom: 1px solid var(--border-color);
    font-size: 12px;
    color: var(--text-secondary);
  }
  
  /* 列表内容 */
  .list-content {
    max-height: 400px;
    overflow-y: auto;
    
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
  
  .list-item {
    display: flex;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
    
    &:last-child {
      border-bottom: none;
    }
    
    &:hover {
      background-color: var(--hover-bg);
    }
  }
  
  /* 列宽设置 */
  .column {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  
  .symbol-column {
    flex: 1;
    min-width: 200px;
  }
  
  .price-column, .change-column, .change-percent-column, .volume-column, .turnover-column {
    width: 100px;
    text-align: right;
  }
  
  /* 符号信息 */
  .symbol-info {
    display: flex;
    align-items: center;
  }
  
  .symbol-icon {
    width: 24px;
    height: 24px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 12px;
    font-size: 12px;
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
  
  .symbol-details {
    display: flex;
    flex-direction: column;
  }
  
  .symbol-code {
    font-size: 14px;
    font-weight: 500;
  }
  
  .symbol-name {
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 2px;
  }
  
  /* 价格单位 */
  .unit {
    font-size: 12px;
    color: var(--text-secondary);
    margin-left: 2px;
  }
  
  /* 涨跌颜色 */
  .positive {
    color: var(--positive-color);
  }
  
  .negative {
    color: var(--negative-color);
  }
  
  /* 空状态 */
  .empty-state {
    padding: 32px 16px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
  
  .empty-message {
    color: var(--text-secondary);
    margin-bottom: 16px;
  }
  
  .add-item-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background-color: var(--button-bg);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    
    &:hover {
      background-color: var(--button-hover-bg);
    }
    
    svg {
      width: 16px;
      height: 16px;
    }
  }
  
  /* 添加分组 */
  .add-group-container {
    display: flex;
    justify-content: center;
    margin-top: 16px;
  }
  
  .add-group-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 20px;
    background-color: transparent;
    color: var(--accent-color);
    border: 1px dashed var(--border-color);
    border-radius: 8px;
    cursor: pointer;
    
    &:hover {
      background-color: var(--hover-bg);
    }
    
    svg {
      width: 16px;
      height: 16px;
    }
  }
  
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
    
    &.group-modal {
      max-width: 400px;
    }
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
    
    /* 使用与前面相同的颜色类 */
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
  
  /* 分组表单 */
  .group-form {
    padding: 16px;
  }
  
  .form-group {
    margin-bottom: 16px;
    
    &:last-child {
      margin-bottom: 0;
    }
    
    label {
      display: block;
      margin-bottom: 8px;
      font-size: 14px;
    }
  }
  
  .form-input, .form-select {
    width: 100%;
    height: 40px;
    padding: 0 12px;
    background-color: var(--input-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-color);
    font-size: 14px;
    
    &:focus {
      outline: none;
      border-color: var(--accent-color);
    }
  }
  
  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding: 16px;
    border-top: 1px solid var(--border-color);
  }
  
  .cancel-button {
    padding: 8px 16px;
    background-color: transparent;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-color);
    font-size: 14px;
    cursor: pointer;
    
    &:hover {
      background-color: var(--hover-bg);
    }
  }
  
  .confirm-button {
    padding: 8px 16px;
    background-color: var(--button-bg);
    border: none;
    border-radius: 4px;
    color: white;
    font-size: 14px;
    cursor: pointer;
    
    &:hover {
      background-color: var(--button-hover-bg);
    }
  }
  </style>