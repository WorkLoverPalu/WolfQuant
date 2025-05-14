<template>
    <div class="left-panel" :style="{ width: `${leftPanelWidth}px` }">
      <!-- 左侧上部分 - 自选列表 -->
      <div class="left-top" :style="{ height: `${leftTopHeight}px` }">
        <!-- 顶部导航栏 -->
        <WatchlistHeader 
          :categories="categories"
          :activeCategory="activeCategory"
          :sortOptions="sortOptions"
          :currentSort="currentSort"
          :showSortMenu="showSortMenu"
          :showChartView="showChartView"
          @setActiveCategory="setActiveCategory"
          @toggleSortMenu="toggleSortMenu"
          @setSort="setSort"
          @toggleChartView="toggleChartView"
          @openPositionSettingsModal="openPositionSettingsModal"
          @openAddGroupModal="openAddGroupModal"
        />
        
        <!-- 分组列表 -->
        <WatchlistGroups 
          :groups="sortedGroups"
          :expandedGroups="expandedGroups"
          :showChartView="showChartView"
          :draggedGroup="draggedGroup"
          @toggleGroup="toggleGroup"
          @selectSymbol="selectSymbol"
          @openAddSymbolModal="openAddSymbolModal"
          @editGroup="editGroup"
          @deleteGroup="deleteGroup"
          @handleDragStart="handleDragStart"
          @handleDragOver="handleDragOver"
          @handleDragEnd="handleDragEnd"
          @handleDrop="handleDrop"
        />
      </div>
      
      <!-- 左侧下部分 - 工具栏 -->
      <div class="horizontal-resizer" @mousedown="startResizeHorizontal"></div>
      
      <WatchlistTools />
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed, inject, onMounted, provide } from 'vue';
  import WatchlistHeader from './watchlist/WatchlistHeader.vue';
  import WatchlistGroups from './watchlist/WatchlistGroups.vue';
  import WatchlistTools from './watchlist/WatchlistTools.vue';
  
  // 接收父组件传递的属性
  const props = defineProps({
    activeCategory: String,
    leftPanelWidth: Number,
    leftTopHeight: Number
  });
  
  // 定义事件
  const emit = defineEmits([
    'startResizeVertical'
  ]);
  
  // 注入全局状态
  const showAddSymbolModal = inject('showAddSymbolModal');
  const showGroupModal = inject('showGroupModal');
  const showPositionModal = inject('showPositionModal');
  const activeCategory = inject('activeCategory');
  
  // 分类数据
  const categories = [
    { id: 'fund', name: '基金' },
    { id: 'stock', name: '股票' },
    { id: 'gold', name: '黄金' },
    { id: 'crypto', name: '数字货币' }
  ];
  
  // 设置激活分类
  const setActiveCategory = (categoryId) => {
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
  const positions = ref({
    'BTCUSD': { cost: 65000, amount: 10000 },
    'ETHUSD': { cost: 3200, amount: 5000 },
    'XAUUSD': { cost: 2300, amount: 8000 },
    '518880': { cost: 2.4, amount: 3000 }
  });
  
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
  const setSort = (sortValue) => {
    currentSort.value = sortValue;
    showSortMenu.value = false;
  };
  
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
  
  // 获取单个商品的持仓信息
  const getItemPosition = (symbol) => {
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
  
  // 展开的分组
  const expandedGroups = ref(['indices', 'crypto', 'gold', 'funds']);
  
  // 切换分组展开/折叠
  const toggleGroup = (groupId) => {
    const index = expandedGroups.value.indexOf(groupId);
    if (index === -1) {
      expandedGroups.value.push(groupId);
    } else {
      expandedGroups.value.splice(index, 1);
    }
  };
  
  // 选中的商品
  const selectedSymbol = ref(null);
  
  // 选择商品
  const selectSymbol = (item) => {
    selectedSymbol.value = item;
  };
  
  // 拖拽排序相关
  const draggedGroup = ref(null);
  
  const handleDragStart = (event, groupId) => {
    draggedGroup.value = groupId;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
      event.dataTransfer.setData('text/plain', groupId);
    }
  };
  
  const handleDragOver = (event, groupId) => {
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = 'move';
    }
  };
  
  const handleDragEnd = () => {
    draggedGroup.value = null;
  };
  
  const handleDrop = (event, targetGroupId) => {
    if (!draggedGroup.value || draggedGroup.value === targetGroupId) return;
    
    const sourceIndex = groups.value.findIndex(g => g.id === draggedGroup.value);
    const targetIndex = groups.value.findIndex(g => g.id === targetGroupId);
    
    if (sourceIndex !== -1 && targetIndex !== -1) {
      const [movedGroup] = groups.value.splice(sourceIndex, 1);
      groups.value.splice(targetIndex, 0, movedGroup);
    }
    
    draggedGroup.value = null;
  };
  
  // 打开添加商品弹窗
  const openAddSymbolModal = (groupId) => {
    showAddSymbolModal.value = true;
  };
  
  // 打开添加分组弹窗
  const openAddGroupModal = () => {
    showGroupModal.value = true;
  };
  
  // 编辑分组
  const editGroup = (group) => {
    showGroupModal.value = true;
  };
  
  // 删除分组
  const deleteGroup = (groupId) => {
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
  
  // 打开持仓设置弹窗
  const openPositionSettingsModal = () => {
    showPositionModal.value = true;
  };
  
  // 开始水平调整大小（左侧上下拖动）
  const startResizeHorizontal = (e) => {
    const isResizingHorizontal = ref(false);
    isResizingHorizontal.value = true;
    
    const handleMouseMove = (e) => {
      if (isResizingHorizontal.value) {
        // 获取左侧面板的位置信息
        const leftPanel = document.querySelector('.left-panel');
        if (leftPanel) {
          const rect = leftPanel.getBoundingClientRect();
          // 计算新高度，确保在合理范围内
          const newHeight = e.clientY - rect.top;
          props.leftTopHeight = Math.max(200, Math.min(newHeight, window.innerHeight - 100));
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
  
  // 提供数据给子组件
  provide('groups', groups);
  provide('positions', positions);
  provide('selectedSymbol', selectedSymbol);
  
  // 初始化
  onMounted(() => {
    // 点击外部关闭排序菜单
    document.addEventListener('click', (e) => {
      if (showSortMenu.value) {
        showSortMenu.value = false;
      }
    });
    
    // 选择默认商品
    if (groups.value.length > 0 && groups.value[0].items.length > 0) {
      selectedSymbol.value = groups.value[0].items[0];
    }
  });
  </script>
  
  <style lang="scss" scoped>
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
  