<template>
  <div class="left-panel" :style="{ width: `${leftPanelWidth}px` }">
    <!-- 左侧上部分 - 自选列表 -->
    <div class="left-top" :style="{ height: `${leftTopHeight}px` }">
      <!-- 顶部导航栏 -->
      <WatchlistHeader :categories="assetStore.categories" :activeCategory="assetStore.activeCategory"
        :sortOptions="sortOptions" :currentSort="currentSort" :showSortMenu="showSortMenu"
        :showChartView="showChartView" @setActiveCategory="setActiveCategory" @toggleSortMenu="toggleSortMenu"
        @setSort="setSort" @toggleChartView="toggleChartView" @openPositionSettingsModal="openPositionSettingsModal"
        @openAddGroupModal="openAddGroupModal" />

      <!-- 分组列表 -->
      <WatchlistGroups :groups="sortedGroups" :expandedGroups="expandedGroups" :showChartView="showChartView"
        :draggedGroup="draggedGroup" @toggleGroup="toggleGroup" @selectSymbol="selectSymbol"
        @openAddSymbolModal="openAddSymbolModal" @editGroup="editGroup" @deleteGroup="confirmDeleteGroup"
        @handleDragStart="handleDragStart" @handleDragOver="handleDragOver" @handleDragEnd="handleDragEnd"
        @handleDrop="handleDrop" />
    </div>

    <!-- 左侧下部分 - 工具栏 -->
    <div class="horizontal-resizer" @mousedown="startResizeHorizontal"></div>

    <WatchlistTools />
  </div>

  <!-- 添加分组弹窗 -->
  <ShowGroupModal v-if="showGroupModal" :show="showGroupModal" :editingGroup="editingGroup" @close="closeGroupModal"
    @saved="handleGroupSaved" />
</template>

<script setup lang="ts">
import { ref, computed, onMounted, provide } from 'vue';
import { useAssetStore } from '../../../stores/assetStore';
import { useUserStore } from '../../../stores/userStore';
import WatchlistHeader from './watchlist/WatchlistHeader.vue';
import WatchlistGroups from './watchlist/WatchlistGroups.vue';
import WatchlistTools from './watchlist/WatchlistTools.vue';
import ShowGroupModal from './GroupModal.vue';
import type { UserGroup, Asset, WatchlistItem } from '../../../stores/assetStore';

// 接收父组件传递的属性
const props = defineProps({
  leftPanelWidth: {
    type: Number,
    default: 300
  },
  leftTopHeight: {
    type: Number,
    default: 500
  }
});

// 定义事件
const emit = defineEmits([
  'startResizeVertical'
]);

// 使用 store
const assetStore = useAssetStore();
const userStore = useUserStore();

// 模态框状态
const showAddSymbolModal = ref(false);
const showGroupModal = ref(false);
const showPositionModal = ref(false);
const editingGroup = ref<UserGroup | null>(null);

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
const setSort = (sortValue: any) => {
  currentSort.value = sortValue;
  showSortMenu.value = false;
};

// 走势图视图
const showChartView = ref(false);

// 切换走势图视图
const toggleChartView = () => {
  showChartView.value = !showChartView.value;
};

// 设置激活分类
const setActiveCategory = (categoryId: any) => {
  assetStore.setActiveCategory(categoryId);
};

// 将后端资产数据转换为前端 WatchlistItem 格式
const convertAssetsToWatchlistItems = (assets: Asset[]): WatchlistItem[] => {
  return assets.map(asset => ({
    symbol: asset.code,
    name: asset.name,
    price: asset.current_price.toFixed(2),
    unit: 'USD', // 默认单位，可以根据资产类型调整
    change: '0.00', // 这些数据可能需要从市场数据中获取
    changePercent: '0.00%',
    volume: '—',
    turnover: '—'
  }));
};

// 将用户组和资产转换为前端分组格式
const convertedGroups = computed(() => {
  // 如果没有用户组数据，使用现有的市场数据分组
  if (assetStore.userGroups.length === 0) {
    return assetStore.filteredGroups;
  }

  // 将用户组转换为前端分组格式
  return assetStore.userGroups.map((group: any) => {
    // 查找该组下的所有资产
    const groupAssets = assetStore.userAssets.filter((asset: any) => asset.group_id === group.id);

    // 查找资产类型
    const assetType = assetStore.assetTypes.find((type: any) => type.id === group.asset_type_id);
    const category = assetType ? mapAssetTypeToCategory(assetType.code) : 'other';

    return {
      id: group.id.toString(),
      name: group.name,
      category,
      description: group.description || '',
      items: convertAssetsToWatchlistItems(groupAssets)
    };
  });
});

// 将资产类型映射到前端分类
const mapAssetTypeToCategory = (assetTypeCode: string): string => {
  const mapping: Record<string, string> = {
    'FUND': 'fund',
    'STOCK': 'stock',
    'GOLD': 'gold',
    'CRYPTO': 'crypto'
  };

  return mapping[assetTypeCode.toUpperCase()] || 'other';
};

// 根据当前分类过滤分组
const filteredGroups = computed(() => {
  if (assetStore.activeCategory === 'all') {
    return convertedGroups.value;
  }
  return convertedGroups.value.filter((group: any) => group.category === assetStore.activeCategory);
});

// 获取单个商品的持仓信息
const getItemPosition = (symbol: any) => {
  const position = assetStore.positions[symbol];
  if (!position || !position.amount) return null;

  // 查找商品当前价格
  let currentPrice = 0;
  for (const group of convertedGroups.value) {
    const item = group.items.find((i: any) => i.symbol === symbol);
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

  return filtered.map((group: any) => {
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
const expandedGroups = ref<string[]>([]);

// 切换分组展开/折叠
const toggleGroup = (groupId: any) => {
  const index = expandedGroups.value.indexOf(groupId);
  if (index === -1) {
    expandedGroups.value.push(groupId);
  } else {
    expandedGroups.value.splice(index, 1);
  }
};

// 选中的商品
const selectedSymbol = ref<WatchlistItem | null>(null);

// 选择商品
const selectSymbol = (item: any) => {
  selectedSymbol.value = item;
  assetStore.selectSymbol(item);
};

// 拖拽排序相关
const draggedGroup = ref(null);

const handleDragStart = (event: any, groupId: any) => {
  draggedGroup.value = groupId;
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move';
    event.dataTransfer.setData('text/plain', groupId);
  }
};

const handleDragOver = (event: any, groupId: any) => {
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move';
  }
};

const handleDragEnd = () => {
  draggedGroup.value = null;
};

const handleDrop = (event: any, targetGroupId: any) => {
  if (!draggedGroup.value || draggedGroup.value === targetGroupId) return;

  // 这里可以实现组的重新排序，但需要与后端 API 对接
  // 目前仅在前端进行视觉上的排序
  draggedGroup.value = null;
};

// 打开添加商品弹窗
const openAddSymbolModal = (groupId: any) => {
  showAddSymbolModal.value = true;
};

// 打开添加分组弹窗
const openAddGroupModal = () => {
  editingGroup.value = null;
  showGroupModal.value = true;
};

// 关闭分组弹窗
const closeGroupModal = () => {
  showGroupModal.value = false;
  editingGroup.value = null;
};

// 处理分组保存
const handleGroupSaved = (group: UserGroup) => {
  // 刷新分组列表
  assetStore.fetchUserGroups();

  // 如果是新建的分组，添加到展开列表
  if (!editingGroup.value) {
    expandedGroups.value.push(group.id.toString());
  }
};

// 编辑分组
const editGroup = (group: any) => {
  // 查找对应的后端分组数据
  const backendGroup = assetStore.userGroups.find((g: any) => g.id.toString() === group.id);
  if (backendGroup) {
    editingGroup.value = backendGroup;
    showGroupModal.value = true;
  }
};

// 确认删除分组
const confirmDeleteGroup = (groupId: any) => {
  if (confirm('确定要删除此分组吗？')) {
    deleteGroup(groupId);
  }
};

// 删除分组
const deleteGroup = async (groupId: any) => {
  try {
    // 将字符串 ID 转换为数字
    const numericId = parseInt(groupId);
    if (isNaN(numericId)) {
      console.error('Invalid group ID:', groupId);
      return;
    }

    // 调用 store 方法删除分组
    await assetStore.deleteUserGroup(numericId);

    // 从展开列表中移除
    const expandedIndex = expandedGroups.value.indexOf(groupId);
    if (expandedIndex !== -1) {
      expandedGroups.value.splice(expandedIndex, 1);
    }
  } catch (err) {
    console.error('Failed to delete group:', err);
    alert('删除分组失败');
  }
};

// 打开持仓设置弹窗
const openPositionSettingsModal = () => {
  showPositionModal.value = true;
};

// 开始水平调整大小（左侧上下拖动）
const startResizeHorizontal = (e: any) => {
  const isResizingHorizontal = ref(false);
  isResizingHorizontal.value = true;

  const handleMouseMove = (e: any) => {
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
provide('selectedSymbol', selectedSymbol);

// 初始化
onMounted(async () => {
  // 加载资产类型和用户分组
  try {
    await assetStore.initAssetData();

    // 初始化展开的分组
    if (assetStore.userGroups.length > 0) {
      expandedGroups.value = assetStore.userGroups.map(g => g.id.toString());
    }

    // 选择默认商品
    if (convertedGroups.value.length > 0 && convertedGroups.value[0].items.length > 0) {
      selectSymbol(convertedGroups.value[0].items[0]);
    }
  } catch (err) {
    console.error('Failed to initialize watchlist:', err);
  }

  // 点击外部关闭排序菜单
  document.addEventListener('click', (e) => {
    if (showSortMenu.value) {
      showSortMenu.value = false;
    }
  });
});
</script>

<style lang="scss" scoped>
/* 左侧面板 */
.left-panel {
  flex: 1;
  height: 100%;
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--borderColor);
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
  background-color: var(--bgColor);
  cursor: ns-resize;
  position: relative;

  &:hover,
  &:active {
    background-color: var(--resizerHoverColor);
  }

  &::after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 30px;
    height: 1px;
    background-color: var( --resizerColor);
  }
}
</style>