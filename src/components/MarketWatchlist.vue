<template>
    <div class="market-watchlist" :class="{ 'light-theme': !isDarkTheme }">
      <div class="watchlist-container">
        <!-- 左侧面板 -->
        <div class="left-panel" :style="{ width: `${leftPanelWidth}px` }">
          <!-- 左侧上部分 - 自选列表 -->
          <div class="left-top" :style="{ height: `${leftTopHeight}px` }">
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
                <button class="action-button" @click="openAddGroupModal" title="新增分组">
                  <PlusIcon class="icon-small" />
                </button>
                
                <div class="sort-dropdown">
                  <button class="sort-button" @click="toggleSortMenu" title="排序选项">
                    <ArrowUpDownIcon class="icon-small" />
                    <ChevronDownIcon class="icon-tiny" />
                  </button>
                  <div v-if="showSortMenu" class="sort-menu">
                    <button 
                      v-for="option in sortOptions" 
                      :key="option.value"
                      class="sort-option"
                      :class="{ active: currentSort === option.value }"
                      @click="setSort(option.value)"
                    >
                      {{ option.label }}
                    </button>
                  </div>
                </div>
                
                <button class="action-button" @click="toggleChartView" title="切换视图">
                  <BarChartIcon v-if="!showChartView" class="icon-small" />
                  <ListIcon v-else class="icon-small" />
                </button>
                
                <button class="action-button" @click="openPositionSettingsModal" title="持仓设置">
                  <WalletIcon class="icon-small" />
                </button>
              </div>
            </div>
            
            <!-- 分组列表 -->
            <div class="watchlist-groups">
              <div 
                v-for="(group, index) in sortedGroups" 
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
                    
                    <div v-if="getGroupPosition(group.id).total > 0" class="group-position">
                      <span class="position-amount">¥{{ formatNumber(getGroupPosition(group.id).total) }}</span>
                      <span 
                        class="position-profit" 
                        :class="getGroupPosition(group.id).profitRate >= 0 ? 'positive' : 'negative'"
                      >
                        {{ getGroupPosition(group.id).profitRate >= 0 ? '+' : '' }}{{ getGroupPosition(group.id).profitRate.toFixed(2) }}%
                      </span>
                    </div>
                  </div>
                  <div class="group-actions">
                    <button class="action-button" @click="openAddSymbolModal(group.id)" title="添加商品">
                      <PlusIcon />
                    </button>
                    <button class="action-button" @click="editGroup(group)" title="编辑分组">
                      <EditIcon />
                    </button>
                    <button class="action-button" @click="deleteGroup(group.id)" title="删除分组">
                      <TrashIcon />
                    </button>
                  </div>
                </div>
                
                <div v-if="expandedGroups.includes(group.id)" class="group-content">
                  <!-- 走势图视图 -->
                  <div v-if="showChartView" class="chart-view">
                    <div 
                      v-for="item in group.items" 
                      :key="item.symbol"
                      class="chart-item"
                      @click="selectSymbol(item)"
                    >
                      <div class="chart-header">
                        <div class="symbol-info">
                          <div class="symbol-icon" :class="getSymbolClass(item.symbol)">
                            {{ getSymbolIcon(item.symbol) }}
                          </div>
                          <div class="symbol-details">
                            <div class="symbol-code">{{ item.symbol }}</div>
                            <div class="symbol-name">{{ item.name }}</div>
                          </div>
                        </div>
                        <div 
                          class="price-info"
                          :class="getChangeClass(item.change)"
                        >
                          <div class="price-value">{{ item.price }}</div>
                          <div class="price-change">{{ item.changePercent }}</div>
                        </div>
                      </div>
                      <div class="chart-container">
                        <!-- 模拟走势图 -->
                        <div class="mini-chart" :class="getChangeClass(item.change)"></div>
                      </div>
                    </div>
                  </div>
                  
                  <!-- 列表视图 -->
                  <div v-else>
                    <!-- 列表标题 -->
                    <div class="list-header">
                      <div class="column symbol-column">商品代码</div>
                      <div class="column price-column">最新价</div>
                      <div class="column change-column">涨跌</div>
                      <div class="column change-percent-column">涨跌%</div>
                      <div class="column position-column">持仓金额</div>
                      <div class="column profit-column">收益</div>
                    </div>
                    
                    <!-- 列表内容 -->
                    <div class="list-content">
                      <div 
                        v-for="item in group.items" 
                        :key="item.symbol"
                        class="list-item"
                        @click="selectSymbol(item)"
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
                        <div class="column position-column">
                          {{ getItemPosition(item.symbol) ? '¥' + formatNumber(getItemPosition(item.symbol).amount) : '—' }}
                        </div>
                        <div 
                          class="column profit-column"
                          :class="getItemPosition(item.symbol) && getItemPosition(item.symbol).profit > 0 ? 'positive' : 
                                 getItemPosition(item.symbol) && getItemPosition(item.symbol).profit < 0 ? 'negative' : ''"
                        >
                          {{ getItemPosition(item.symbol) ? (getItemPosition(item.symbol).profit > 0 ? '+' : '') + formatNumber(getItemPosition(item.symbol).profit) : '—' }}
                        </div>
                      </div>
                      
                      <!-- 空状态 -->
                      <div v-if="group.items.length === 0" class="empty-state">
                        <div class="empty-message">该分组暂无数据</div>
                        <button class="add-item-button" @click="openAddSymbolModal(group.id)">
                          <PlusIcon />
                          添加商品
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <!-- 左侧下部分 - 工具栏 -->
          <div class="horizontal-resizer" @mousedown="startResizeHorizontal"></div>
          
          <div class="left-bottom">
            <div class="tools-bar">
              <button class="tool-button">加密货币对游戏器</button>
              <button class="tool-button">Pine编辑器</button>
              <button class="tool-button">策略测试器</button>
              <button class="tool-button">回放交易面板</button>
              <button class="tool-button">交易面板</button>
              
              <div class="tools-right">
                <div class="zoom-control">
                  <span>上下拖动缩放</span>
                </div>
                <div class="view-controls">
                  <button class="view-button">全屏</button>
                  <button class="view-button">隐藏</button>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 垂直分隔线 - 可拖动调整宽度 -->
        <div class="vertical-resizer" @mousedown="startResizeVertical"></div>
        
        <!-- 右侧面板 - 图表区域 -->
        <div class="right-panel">
          <!-- 右侧上部分 - 主图表 -->
          <div class="right-top" :style="{ height: `${rightTopHeight}px` }">
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
          </div>
          
          <!-- 水平分隔线 - 可拖动调整高度 -->
          <div class="horizontal-resizer" @mousedown="startResizeHorizontalRight"></div>
          
          <!-- 右侧下部分 - 副图表 -->
          <div class="right-bottom">
            <div class="sub-chart-tabs">
              <button class="sub-chart-tab active">成交量</button>
              <button class="sub-chart-tab">MACD</button>
              <button class="sub-chart-tab">RSI</button>
              <button class="sub-chart-tab">KDJ</button>
              <button class="sub-chart-tab">
                <PlusIcon class="icon-tiny" />
                <span>添加指标</span>
              </button>
            </div>
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
          </div>
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
      
      <!-- 持仓设置弹窗 -->
      <div v-if="showPositionModal" class="modal-overlay" @click="closePositionModal">
        <div class="modal-container position-modal" @click.stop>
          <div class="modal-header">
            <h3>持仓金额设置</h3>
            <button class="close-button" @click="closePositionModal">
              <XIcon />
            </button>
          </div>
          
          <div class="position-list">
            <div v-for="group in filteredGroups" :key="group.id" class="position-group">
              <div class="position-group-header">{{ group.name }}</div>
              
              <div v-for="item in group.items" :key="item.symbol" class="position-item">
                <div class="position-symbol">
                  <div class="symbol-icon small" :class="getSymbolClass(item.symbol)">
                    {{ getSymbolIcon(item.symbol) }}
                  </div>
                  <div class="symbol-details">
                    <div class="symbol-code">{{ item.symbol }}</div>
                    <div class="symbol-name">{{ item.name }}</div>
                  </div>
                </div>
                
                <div class="position-inputs">
                  <div class="input-group">
                    <label>持仓成本</label>
                    <input 
                      type="number" 
                      class="position-input" 
                      placeholder="输入成本价" 
                      v-model="getPositionInput(item.symbol).cost"
                    />
                  </div>
                  <div class="input-group">
                    <label>持仓金额</label>
                    <input 
                      type="number" 
                      class="position-input" 
                      placeholder="输入金额" 
                      v-model="getPositionInput(item.symbol).amount"
                    />
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <div class="modal-footer">
            <button class="cancel-button" @click="closePositionModal">取消</button>
            <button class="confirm-button" @click="savePositions">保存</button>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed, onMounted, watch, onUnmounted } from 'vue';
  import { 
    ChevronDownIcon, 
    ChevronRightIcon, 
    PlusIcon, 
    EditIcon, 
    TrashIcon, 
    XIcon, 
    SearchIcon,
    ArrowUpDownIcon,
    BarChartIcon,
    ListIcon,
    WalletIcon,
    LineChartIcon,
    Settings2Icon,
    BarChart2Icon
  } from 'lucide-vue-next';
  
  // 主题状态 - 与系统主题保持一致
  const isDarkTheme = ref(window.matchMedia('(prefers-color-scheme: dark)').matches);
  
  // 监听系统主题变化
  onMounted(() => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handleThemeChange = (e: MediaQueryListEvent) => {
      isDarkTheme.value = e.matches;
    };
    
    mediaQuery.addEventListener('change', handleThemeChange);
    
    // 组件卸载时移除监听器
    onUnmounted(() => {
      mediaQuery.removeEventListener('change', handleThemeChange);
    });
  });
  
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
  
  // 持仓数据
  const positions = ref<Record<string, { cost: number; amount: number }>>({
    'BTCUSD': { cost: 65000, amount: 10000 },
    'ETHUSD': { cost: 3200, amount: 5000 },
    'XAUUSD': { cost: 2300, amount: 8000 },
    '518880': { cost: 2.4, amount: 3000 }
  });
  
  // 持仓输入临时存储
  const positionInputs = ref<Record<string, { cost: string; amount: string }>>({});
  
  // 获取持仓输入
  const getPositionInput = (symbol: string) => {
    if (!positionInputs.value[symbol]) {
      const position = positions.value[symbol] || { cost: 0, amount: 0 };
      positionInputs.value[symbol] = {
        cost: position.cost ? position.cost.toString() : '',
        amount: position.amount ? position.amount.toString() : ''
      };
    }
    return positionInputs.value[symbol];
  };
  
  // 获取单个商品的持仓信息
  const getItemPosition = (symbol: string) => {
    const position = positions.value[symbol];
    if (!position || !position.amount) return null;
    
    // 查找商品当前价格
    let currentPrice = 0;
    for (const group of groups.value) {
      const item = group.items.find(i => i.symbol === symbol);
      if (item) {
        currentPrice = parseFloat(item.price.replace(',', ''));
        break;
      }
    }
    
    if (!currentPrice) return null;
    
    const cost = position.cost * position.amount / currentPrice;
    const currentValue = position.amount;
    const profit = currentValue - cost;
    
    return {
      cost,
      amount: position.amount,
      profit,
      profitRate: (profit / cost) * 100
    };
  };
  
  // 获取分组的持仓信息
  const getGroupPosition = (groupId: string) => {
    const group = groups.value.find(g => g.id === groupId);
    if (!group) return { total: 0, profit: 0, profitRate: 0 };
    
    let totalCost = 0;
    let totalValue = 0;
    
    for (const item of group.items) {
      const position = getItemPosition(item.symbol);
      if (position) {
        totalCost += position.cost;
        totalValue += position.amount;
      }
    }
    
    const profit = totalValue - totalCost;
    const profitRate = totalCost > 0 ? (profit / totalCost) * 100 : 0;
    
    return {
      total: totalValue,
      profit,
      profitRate
    };
  };
  
  // 排序选项
  const sortOptions = [
    { value: 'default', label: '默认排序' },
    { value: 'change-asc', label: '涨跌幅 ↑' },
    { value: 'change-desc', label: '涨跌幅 ↓' },
    { value: 'position-asc', label: '持仓金额 ↑' },
    { value: 'position-desc', label: '持仓金额 ↓' }
  ];
  
  // 当前排序方式
  const currentSort = ref('default');
  const showSortMenu = ref(false);
  
  // 切换排序菜单
  const toggleSortMenu = () => {
    showSortMenu.value = !showSortMenu.value;
  };
  
  // 设置排序方式
  const setSort = (sortValue: string) => {
    currentSort.value = sortValue;
    showSortMenu.value = false;
  };
  
  // 排序后的分组
  const sortedGroups = computed(() => {
    const filtered = filteredGroups.value;
    
    if (currentSort.value === 'default') {
      return filtered;
    }
    
    return filtered.map(group => {
      const sortedItems = [...group.items].sort((a, b) => {
        if (currentSort.value.startsWith('change')) {
          const changeA = parseFloat(a.changePercent.replace('%', '').replace('+', ''));
          const changeB = parseFloat(b.changePercent.replace('%', '').replace('+', ''));
          return currentSort.value === 'change-asc' ? changeA - changeB : changeB - changeA;
        } else if (currentSort.value.startsWith('position')) {
          const posA = getItemPosition(a.symbol)?.amount || 0;
          const posB = getItemPosition(b.symbol)?.amount || 0;
          return currentSort.value === 'position-asc' ? posA - posB : posB - posA;
        }
        return 0;
      });
      
      return {
        ...group,
        items: sortedItems
      };
    });
  });
  
  // 走势图视图
  const showChartView = ref(false);
  
  // 切换走势图视图
  const toggleChartView = () => {
    showChartView.value = !showChartView.value;
  };
  
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
  
  // 格式化数字
  const formatNumber = (num: number) => {
    return num.toLocaleString('zh-CN', { maximumFractionDigits: 2 });
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
  const currentTargetGroup = ref<string | null>(null);
  
  const openAddSymbolModal = (groupId?: string) => {
    showAddSymbolModal.value = true;
    modalActiveCategory.value = activeCategory.value;
    currentTargetGroup.value = groupId || null;
    searchSymbols();
  };
  
  const closeAddSymbolModal = () => {
    showAddSymbolModal.value = false;
    searchQuery.value = '';
    currentTargetGroup.value = null;
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
  
  // 持仓设置弹窗
  const showPositionModal = ref(false);
  
  const openPositionSettingsModal = () => {
    // 初始化输入数据
    positionInputs.value = {};
    showPositionModal.value = true;
  };
  
  const closePositionModal = () => {
    showPositionModal.value = false;
  };
  
  const savePositions = () => {
    // 保存持仓设置
    for (const [symbol, input] of Object.entries(positionInputs.value)) {
      const cost = parseFloat(input.cost);
      const amount = parseFloat(input.amount);
      
      if (!isNaN(cost) && !isNaN(amount) && amount > 0) {
        positions.value[symbol] = { cost, amount };
      } else if (amount === 0 || isNaN(amount)) {
        // 如果金额为0或无效，则删除持仓
        delete positions.value[symbol];
      }
    }
    
    closePositionModal();
  };
  
  // 选中的商品
  const selectedSymbol = ref<any>(null);
  
  // 选择商品
  const selectSymbol = (item: any) => {
    selectedSymbol.value = item;
  };
  
  // 面板尺寸调整
  const leftPanelWidth = ref(350); // 左侧面板宽度
  const leftTopHeight = ref(500); // 左侧上部分高度
  const rightTopHeight = ref(500); // 右侧上部分高度
  const isResizingVertical = ref(false);
  const isResizingHorizontal = ref(false);
  const isResizingHorizontalRight = ref(false);
  
  // 开始垂直调整大小（左右拖动）
  const startResizeVertical = (e: MouseEvent) => {
    isResizingVertical.value = true;
    
    const handleMouseMove = (e: MouseEvent) => {
      if (isResizingVertical.value) {
        // 计算新宽度，确保在合理范围内
        const newWidth = e.clientX;
        leftPanelWidth.value = Math.max(250, Math.min(newWidth, window.innerWidth * 0.7));
      }
    };
    
    const handleMouseUp = () => {
      isResizingVertical.value = false;
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
    
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
    
    // 防止拖拽时选中文本
    e.preventDefault();
  };
  
  // 开始水平调整大小（左侧上下拖动）
  const startResizeHorizontal = (e: MouseEvent) => {
    isResizingHorizontal.value = true;
    
    const handleMouseMove = (e: MouseEvent) => {
      if (isResizingHorizontal.value) {
        // 获取左侧面板的位置信息
        const leftPanel = document.querySelector('.left-panel');
        if (leftPanel) {
          const rect = leftPanel.getBoundingClientRect();
          // 计算新高度，确保在合理范围内
          const newHeight = e.clientY - rect.top;
          leftTopHeight.value = Math.max(200, Math.min(newHeight, window.innerHeight - 100));
        }
      }
    };
    
    const handleMouseUp = () => {
      isResizingHorizontal.value = false;
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
    
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
    
    // 防止拖拽时选中文本
    e.preventDefault();
  };
  
  // 开始水平调整大小（右侧上下拖动）
  const startResizeHorizontalRight = (e: MouseEvent) => {
    isResizingHorizontalRight.value = true;
    
    const handleMouseMove = (e: MouseEvent) => {
      if (isResizingHorizontalRight.value) {
        // 获取右侧面板的位置信息
        const rightPanel = document.querySelector('.right-panel');
        if (rightPanel) {
          const rect = rightPanel.getBoundingClientRect();
          // 计算新高度，确保在合理范围内
          const newHeight = e.clientY - rect.top;
          rightTopHeight.value = Math.max(200, Math.min(newHeight, window.innerHeight - 100));
        }
      }
    };
    
    const handleMouseUp = () => {
      isResizingHorizontalRight.value = false;
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
    
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
    
    // 防止拖拽时选中文本
    e.preventDefault();
  };
  
  // 初始化
  onMounted(() => {
    // 可以在这里加载保存的数据
    console.log('市场行情列表组件已加载');
    
    // 点击外部关闭排序菜单
    document.addEventListener('click', (e) => {
      if (showSortMenu.value) {
        showSortMenu.value = false;
      }
    });
    
    // 设置初始面板高度
    leftTopHeight.value = window.innerHeight * 0.6;
    rightTopHeight.value = window.innerHeight * 0.6;
    
    // 选择默认商品
    if (groups.value.length > 0 && groups.value[0].items.length > 0) {
      selectedSymbol.value = groups.value[0].items[0];
    }
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
    --tools-bg: #0a0a0a;
    --resizer-color: #444444;
    --resizer-hover-color: #2962ff;
    --chart-grid-color: rgba(255, 255, 255, 0.05);
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
    --tools-bg: #f0f0f0;
    --resizer-color: #dddddd;
    --resizer-hover-color: #1a73e8;
    --chart-grid-color: rgba(0, 0, 0, 0.05);
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
  
  .watchlist-container {
    display: flex;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }
  
  /* 左侧面板 */
  .left-panel {
    flex: 1;
    height: 100%;
    display: flex;
    flex-direction: column;
    border-right: 1px solid var(--border-color);
    position: relative;
  }
  
  .left-top {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  
  .left-bottom {
    flex: 1;
    background-color: var(--tools-bg);
    border-top: 1px solid var(--border-color);
    overflow: hidden;
  }
  
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
  
  /* 分隔线 */
  .vertical-resizer {
    width: 5px;
    height: 100%;
    background-color: var(--bg-color);
    cursor: ew-resize;
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
      width: 1px;
      height: 30px;
      background-color: var(--resizer-color);
    }
  }
  
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
  
  /* 顶部导航栏 */
  .watchlist-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 16px;
    height: 48px;
    background-color: var(--header-bg);
    border-bottom: 1px solid var(--border-color);
    flex-shrink: 0;
  }
  
  .category-tabs {
    display: flex;
    gap: 8px;
  }
  
  .category-tab {
    padding: 6px 12px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 13px;
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
    gap: 4px;
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
    
    .icon-small {
      width: 16px;
      height: 16px;
    }
    
    .icon-tiny {
      width: 14px;
      height: 14px;
    }
  }
  
  /* 排序下拉菜单 */
  .sort-dropdown {
    position: relative;
  }
  
  .sort-button {
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
  }
  
  .sort-menu {
    position: absolute;
    top: 100%;
    right: 0;
    width: 150px;
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    z-index: 10;
    margin-top: 4px;
    overflow: hidden;
  }
  
  .sort-option {
    width: 100%;
    padding: 8px 12px;
    text-align: left;
    background: transparent;
    border: none;
    color: var(--text-color);
    font-size: 13px;
    cursor: pointer;
    
    &:hover {
      background-color: var(--hover-bg);
    }
    
    &.active {
      background-color: var(--active-bg);
      font-weight: 500;
    }
  }
  
  /* 分组列表 */
  .watchlist-groups {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
    
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
    margin-bottom: 8px;
    border-radius: 6px;
    background-color: var(--card-bg);
    overflow: hidden;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
    
    &.is-dragging {
      opacity: 0.7;
    }
  }
  
  .group-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    background-color: var(--header-bg);
    border-bottom: 1px solid var(--border-color);
  }
  
  .group-title {
    display: flex;
    align-items: center;
    font-weight: 500;
    cursor: pointer;
    font-size: 13px;
    
    svg {
      width: 16px;
      height: 16px;
      margin-right: 6px;
    }
  }
  
  .group-position {
    display: flex;
    align-items: center;
    margin-left: 8px;
    font-size: 12px;
    font-weight: normal;
  }
  
  .position-amount {
    color: var(--text-secondary);
  }
  
  .position-profit {
    margin-left: 6px;
    
    &.positive {
      color: var(--positive-color);
    }
    
    &.negative {
      color: var(--negative-color);
    }
  }
  
  .group-actions {
    display: flex;
    gap: 4px;
  }
  
  .action-button {
    width: 24px;
    height: 24px;
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
      width: 14px;
      height: 14px;
    }
  }
  
  /* 列表标题 */
  .list-header {
    display: flex;
    padding: 6px 12px;
    border-bottom: 1px solid var(--border-color);
    font-size: 11px;
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
    padding: 8px 12px;
    border-bottom: 1px solid var(--border-color);
    font-size: 12px;
    cursor: pointer;
    
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
    min-width: 120px;
  }
  
  .price-column, .change-column, .change-percent-column {
    width: 60px;
    text-align: right;
  }
  
  .position-column, .profit-column {
    width: 70px;
    text-align: right;
  }
  
  /* 符号信息 */
  .symbol-info {
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
    
    &.small {
      width: 18px;
      height: 18px;
      font-size: 9px;
      margin-right: 6px;
    }
    
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
  
  .symbol-code {
    font-size: 13px;
    font-weight: 500;
  }
  
  .symbol-name {
    font-size: 11px;
    color: var(--text-secondary);
    margin-top: 1px;
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
      color: var(--positive-color);
    }
    
    &.negative {
      color: var(--negative-color);
    }
  }
  
  /* 价格单位 */
  .unit {
    font-size: 11px;
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
  
  /* 走势图视图 */
  .chart-view {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 8px;
    padding: 8px;
  }
  
  .chart-item {
    background-color: var(--card-bg);
    border-radius: 4px;
    border: 1px solid var(--border-color);
    overflow: hidden;
    cursor: pointer;
    
    &:hover {
      border-color: var(--accent-color);
    }
  }
  
  .chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px;
    border-bottom: 1px solid var(--border-color);
  }
  
  .price-info {
    text-align: right;
  }
  
  .price-value {
    font-weight: 500;
    font-size: 13px;
  }
  
  .price-change {
    font-size: 11px;
    margin-top: 2px;
  }
  
  .chart-container {
    height: 80px;
    padding: 8px;
  }
  
  .mini-chart {
    width: 100%;
    height: 100%;
    background-image: linear-gradient(to right, var(--chart-grid-color) 1px, transparent 1px),
                      linear-gradient(to bottom, var(--chart-grid-color) 1px, transparent 1px);
    background-size: 20px 20px;
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
      background-size: 100% 70%;
    }
    
    &.positive::after {
      background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 50'%3E%3Cpath d='M0,35 C10,30 20,40 30,25 C40,10 50,15 60,5 C70,15 80,20 90,15 L100,10' stroke='%234caf50' fill='none' stroke-width='2'/%3E%3C/svg%3E");
    }
    
    &.negative::after {
      background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 50'%3E%3Cpath d='M0,15 C10,20 20,10 30,25 C40,40 50,35 60,45 C70,35 80,30 90,35 L100,40' stroke='%23f44336' fill='none' stroke-width='2'/%3E%3C/svg%3E");
    }
  }
  
  /* 空状态 */
  .empty-state {
    padding: 24px 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
  
  .empty-message {
    color: var(--text-secondary);
    margin-bottom: 12px;
    font-size: 13px;
  }
  
  .add-item-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background-color: var(--button-bg);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    
    &:hover {
      background-color: var(--button-hover-bg);
    }
    
    svg {
      width: 14px;
      height: 14px;
    }
  }
  
  /* 工具栏 */
  .tools-bar {
    display: flex;
    align-items: center;
    padding: 0 12px;
    height: 36px;
    background-color: var(--tools-bg);
    border-bottom: 1px solid var(--border-color);
    overflow-x: auto;
    
    &::-webkit-scrollbar {
      height: 0;
    }
  }
  
  .tool-button {
    padding: 0 12px;
    height: 36px;
    background: transparent;
    border: none;
    color: var(--text-color);
    font-size: 12px;
    white-space: nowrap;
    cursor: pointer;
    
    &:hover {
      background-color: var(--hover-bg);
    }
  }
  
  .tools-right {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 16px;
  }
  
  .zoom-control {
    color: var(--accent-color);
    font-size: 12px;
  }
  
  .view-controls {
    display: flex;
    gap: 8px;
  }
  
  .view-button {
    padding: 0 8px;
    height: 24px;
    background: transparent;
    border: none;
    color: var(--text-color);
    font-size: 12px;
    cursor: pointer;
    
    &:hover {
      text-decoration: underline;
    }
  }
  
  /* 图表区域 */
  .chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background-color: var(--header-bg);
    border-bottom: 1px solid var(--border-color);
  }
  
  .chart-symbol-info {
    display: flex;
    align-items: center;
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
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-color);
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
    background-color: var(--bg-color);
  }
  
  .no-symbol-selected {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    
    .large-icon {
      width: 48px;
      height: 48px;
      margin-bottom: 16px;
      opacity: 0.5;
    }
    
    p {
      font-size: 14px;
    }
    
    &.small {
      .large-icon {
        width: 32px;
        height: 32px;
        margin-bottom: 8px;
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
  
  .chart-canvas {
    width: 100%;
    height: 100%;
    background-image: 
      linear-gradient(to right, var(--chart-grid-color) 1px, transparent 1px),
      linear-gradient(to bottom, var(--chart-grid-color) 1px, transparent 1px);
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
  
  .volume-chart {
    width: 100%;
    height: 100%;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 50'%3E%3Crect x='5' y='30' width='3' height='15' fill='%234caf50' /%3E%3Crect x='10' y='25' width='3' height='20' fill='%234caf50' /%3E%3Crect x='15' y='35' width='3' height='10' fill='%23f44336' /%3E%3Crect x='20' y='20' width='3' height='25' fill='%234caf50' /%3E%3Crect x='25' y='15' width='3' height='30' fill='%234caf50' /%3E%3Crect x='30' y='25' width='3' height='20' fill='%23f44336' /%3E%3Crect x='35' y='30' width='3' height='15' fill='%23f44336' /%3E%3Crect x='40' y='35' width='3' height='10' fill='%23f44336' /%3E%3Crect x='45' y='25' width='3' height='20' fill='%234caf50' /%3E%3Crect x='50' y='20' width='3' height='25' fill='%234caf50' /%3E%3Crect x='55' y='15' width='3' height='30' fill='%234caf50' /%3E%3Crect x='60' y='10' width='3' height='35' fill='%234caf50' /%3E%3Crect x='65' y='20' width='3' height='25' fill='%23f44336' /%3E%3Crect x='70' y='25' width='3' height='20' fill='%23f44336' /%3E%3Crect x='75' y='30' width='3' height='15' fill='%23f44336' /%3E%3Crect x='80' y='25' width='3' height='20' fill='%234caf50' /%3E%3Crect x='85' y='20' width='3' height='25' fill='%234caf50' /%3E%3Crect x='90' y='25' width='3' height='20' fill='%23f44336' /%3E%3Crect x='95' y='30' width='3' height='15' fill='%23f44336' /%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: center;
    background-size: 95% 90%;
  }
  
  .sub-chart-tabs {
    display: flex;
    align-items: center;
    padding: 0 16px;
    height: 36px;
    background-color: var(--header-bg);
    border-bottom: 1px solid var(--border-color);
    overflow-x: auto;
    
    &::-webkit-scrollbar {
      height: 0;
    }
  }
  
  .sub-chart-tab {
    padding: 0 12px;
    height: 36px;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 12px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 4px;
    
    &:hover {
      color: var(--text-color);
    }
    
    &.active {
      color: var(--accent-color);
      position: relative;
      
      &::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 0;
        width: 100%;
        height: 2px;
        background-color: var(--accent-color);
      }
    }
    
    .icon-tiny {
      width: 12px;
      height: 12px;
    }
  }
  
  .sub-chart {
    flex: 1;
    overflow: hidden;
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
    
    &.position-modal {
      max-width: 500px;
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
  
  /* 持仓设置样式 */
  .position-list {
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
  
  .position-group {
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 8px;
    margin-bottom: 8px;
    
    &:last-child {
      border-bottom: none;
      margin-bottom: 0;
    }
  }
  
  .position-group-header {
    padding: 12px 16px 8px;
    font-weight: 500;
    color: var(--accent-color);
  }
  
  .position-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    
    &:hover {
      background-color: var(--hover-bg);
    }
  }
  
  .position-symbol {
    display: flex;
    align-items: center;
  }
  
  .position-inputs {
    display: flex;
    gap: 12px;
  }
  
  .input-group {
    display: flex;
    flex-direction: column;
    
    label {
      font-size: 12px;
      color: var(--text-secondary);
      margin-bottom: 4px;
    }
  }
  
  .position-input {
    width: 100px;
    height: 32px;
    padding: 0 8px;
    background-color: var(--input-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-color);
    font-size: 13px;
    
    &:focus {
      outline: none;
      border-color: var(--accent-color);
    }
  }
  </style>