<template>
  <div class="left-panel" :style="{ width: `${leftPanelWidth}px` }">
    <!-- 左侧上部分 - 自选列表 -->
    <div class="left-top" :style="{ height: `${leftTopHeight}px` }">
      <!-- 顶部导航栏 -->
      <WatchlistHeader :assetTypes="assetStore.assetTypes" :activeCategory="assetStore.activeCategory"
        :sortOptions="sortOptions" :currentSort="currentSort" :showSortMenu="showSortMenu"
        :showChartView="showChartView" @setActiveCategory="setActiveCategory" @toggleSortMenu="toggleSortMenu"
        @setSort="setSort" @toggleChartView="toggleChartView" @openPositionSettingsModal="openPositionSettingsModal"
        @openAddGroupModal="openAddGroupModal" @openAssetTypeSettings="openAssetTypeSettings" @refresh="getData" />

      <!-- 分组列表 -->
      <WatchlistGroups :groups="sortedGroups" :expandedGroups="expandedGroups" :showChartView="showChartView"
        :draggedGroup="draggedGroup" @toggleGroup="toggleGroup" @selectSymbol="selectSymbol"
        @openAddSymbolModal="openAddSymbolModal" @editGroup="editGroup" @deleteGroup="confirmDeleteGroup"
        @handleDragStart="handleDragStart" @handleDragOver="handleDragOver" @handleDragEnd="handleDragEnd"
        @handleDrop="handleDrop" @editPosition="openPositionEditModal" />
    </div>

    <!-- 左侧下部分 - 工具栏 -->
    <div class="horizontal-resizer" @mousedown="startResizeHorizontal"></div>

    <WatchlistTools />
  </div>

  <!-- 添加分组弹窗 -->
  <GroupModal v-if="showGroupModal" :show="showGroupModal" :editingGroup="editingGroup" @close="closeGroupModal"
    @saved="handleGroupSaved" />

  <!-- 添加资产弹窗 -->
  <AddAssetModal v-if="showAddSymbolModal && selectedGroupId" :show="showAddSymbolModal" :groupId="selectedGroupId"
    @close="closeAddSymbolModal" @added="handleAssetAdded" />

  <!-- 持仓编辑弹窗 -->
  <PositionEditModal v-if="showPositionEditModal && selectedAsset" :show="showPositionEditModal" :asset="selectedAsset"
    @close="closePositionEditModal" @saved="handlePositionSaved" />

  <!-- 资产类型设置弹窗 -->
  <AssetTypeSettingsModal v-if="showAssetTypeSettingsModal && selectedAssetTypeId" :show="showAssetTypeSettingsModal"
    :assetTypeId="selectedAssetTypeId" @close="closeAssetTypeSettingsModal" @openAddGroup="openAddGroupModal"
    @editGroup="editGroup" @deleteGroup="confirmDeleteGroup" @editPosition="openPositionEditModal" />

  <!-- 确认删除弹窗 -->
  <ConfirmDialog v-if="showDeleteConfirm" :show="showDeleteConfirm" title="删除确认" message="确定要删除此分组吗？删除后无法恢复。"
    confirmText="删除" cancelText="取消" :danger="true" @confirm="handleDeleteConfirm" @cancel="cancelDeleteConfirm" />
</template>

<script setup lang="ts">
import { ref, computed, onMounted, provide,onBeforeMount ,} from 'vue';
import { useAssetStore } from '../../../stores/assetStore';
import { useUserStore } from '../../../stores/userStore';
import WatchlistHeader from './watchlist/WatchlistHeader.vue';
import WatchlistGroups from './watchlist/WatchlistGroups.vue';
import WatchlistTools from './watchlist/WatchlistTools.vue';
import GroupModal from './GroupModal.vue';
import AddAssetModal from './watchlist/AddAssetModal.vue';
import PositionEditModal from './watchlist/PositionEditModal.vue';
import AssetTypeSettingsModal from './watchlist/AssetTypeSettingsModal.vue';
import ConfirmDialog from '../../../components/dialog/ConfirmDialog.vue';
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
const showPositionEditModal = ref(false);
const showAssetTypeSettingsModal = ref(false);
const showDeleteConfirm = ref(false);
const editingGroup = ref<UserGroup | null>(null);
const selectedGroupId = ref<number | null>(null);
const selectedAsset = ref<WatchlistItem | null>(null);
const selectedAssetTypeId = ref<number | null>(null);
const groupToDelete = ref<string | number | null>(null);

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
const toggleSortMenu = (bool: boolean) => {
  showSortMenu.value = bool;
};

// 设置排序方式
const setSort = (sortValue: string) => {
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
const setActiveCategory = (categoryId: string) => {
  assetStore.setActiveCategory(categoryId);
};

// 将后端资产数据转换为前端 WatchlistItem 格式
const convertAssetsToWatchlistItems = (assets: Asset[]): WatchlistItem[] => {
  return assets.map(asset => ({
    symbol: asset.code,
    name: asset.name,
    price: asset.current_price ? asset.current_price.toFixed(2) : '0.00',
    unit: 'USD', // 默认单位，可以根据资产类型调整
    change: '0.00', // 这些数据可能需要从市场数据中获取
    changePercent: '0.00%',
    volume: '—',
    turnover: '—'
  }));
};

// 将资产类型代码映射到前端分类
const mapAssetTypeToCategory = (assetTypeName: string): string => {
  const mapping: Record<string, string> = {
    'FUND': 'fund',
    'STOCK': 'stock',
    'GOLD': 'gold',
    'CRYPTO': 'crypto'
  };

  return mapping[assetTypeName] || 'other';
};

// 将用户组和资产转换为前端分组格式
const convertedGroups = computed(() => {
  // 如果没有用户组数据，返回空数组
  if (!assetStore.userGroups || assetStore.userGroups.length === 0) {
    return [];
  }

  // 将用户组转换为前端分组格式
  return assetStore.userGroups.map((group) => {
    // 查找该组下的所有资产
    const groupAssets = assetStore.userAssets.filter((asset) => asset.group_id === group.id);

    // 查找资产类型
    const assetType = assetStore.assetTypes.find((type) => type.id === group.asset_type_id);
    const assetTypeName = assetType ? assetType.name : 'OTHER';
    const category = mapAssetTypeToCategory(assetTypeName);

    return {
      id: group.id.toString(),
      name: group.name,
      category,
      description: group.description || '',
      items: convertAssetsToWatchlistItems(groupAssets)
    };
  });
});

// 根据当前分类过滤分组
const filteredGroups = computed(() => {
  if (assetStore.activeCategory === 'all') {
    return convertedGroups.value;
  }
  return convertedGroups.value.filter((group) => group.category === assetStore.activeCategory);
});

// 获取单个商品的持仓信息
const getItemPosition = (symbol: string) => {
  const position = assetStore.positions[symbol];
  if (!position || !position.amount) return null;

  // 查找商品当前价格
  let currentPrice = 0;
  const asset = assetStore.userAssets.find((asset) => asset.code === symbol);

  if (asset && asset.current_price) {
    currentPrice = asset.current_price;
  } else {
    // 如果在用户资产中找不到，尝试在所有分组中查找
    for (const group of convertedGroups.value) {
      const item = group.items.find((i) => i.symbol === symbol);
      if (item) {
        currentPrice = parseFloat(item.price.replace(',', ''));
        break;
      }
    }
  }

  if (!currentPrice) return null;

  const cost = position.cost * position.amount;
  const currentValue = position.amount * currentPrice;
  const profit = currentValue - cost;

  return {
    cost,
    amount: position.amount,
    currentValue,
    profit,
    profitRate: cost > 0 ? (profit / cost) * 100 : 0
  };
};

// 排序后的分组
const sortedGroups = computed(() => {
  const filtered = filteredGroups.value;

  if (currentSort.value === 'default') {
    return filtered;
  }

  return filtered.map((group) => {
    const sortedItems = [...group.items].sort((a, b) => {
      if (currentSort.value.startsWith('change')) {
        const changeA = parseFloat(a.changePercent.replace('%', '').replace('+', ''));
        const changeB = parseFloat(b.changePercent.replace('%', '').replace('+', ''));
        return currentSort.value === 'change-asc' ? changeA - changeB : changeB - changeA;
      } else if (currentSort.value.startsWith('position')) {
        const posA = getItemPosition(a.symbol)?.currentValue || 0;
        const posB = getItemPosition(b.symbol)?.currentValue || 0;
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
const toggleGroup = (groupId: string) => {
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
const selectSymbol = (item: WatchlistItem) => {
  selectedSymbol.value = item;
  assetStore.selectSymbol(item);
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

  // 这里可以实现组的重新排序，但需要与后端 API 对接
  // 目前仅在前端进行视觉上的排序
  draggedGroup.value = null;
};

// 打开添加商品弹窗
const openAddSymbolModal = (groupId: string) => {
  selectedGroupId.value = parseInt(groupId);
  showAddSymbolModal.value = true;
};

// 关闭添加商品弹窗
const closeAddSymbolModal = () => {
  showAddSymbolModal.value = false;
  selectedGroupId.value = null;
};

// 处理资产添加
const handleAssetAdded = (asset: Asset) => {
  // 刷新资产列表
  assetStore.fetchUserAssets(undefined, asset.group_id);
};

// 打开添加分组弹窗
const openAddGroupModal = (assetTypeId?: number) => {
  editingGroup.value = null;
  showGroupModal.value = true;

  // 关闭资产类型设置弹窗
  if (showAssetTypeSettingsModal.value) {
    showAssetTypeSettingsModal.value = false;
  }
};

// 关闭分组弹窗
const closeGroupModal = () => {
  showGroupModal.value = false;
  editingGroup.value = null;

  // 如果是从资产类型设置弹窗打开的，重新打开资产类型设置弹窗
  if (selectedAssetTypeId.value) {
    showAssetTypeSettingsModal.value = true;
  }
};

// 处理分组保存
const handleGroupSaved = (group: UserGroup) => {
  // 如果是新建的分组，添加到展开列表
  if (!editingGroup.value || editingGroup.value.id === 0) {
    expandedGroups.value.push(group.id.toString());
  }

  // 如果是从资产类型设置弹窗打开的，重新打开资产类型设置弹窗
  if (selectedAssetTypeId.value) {
    showAssetTypeSettingsModal.value = true;
  }
};

// 编辑分组
const editGroup = (group: any) => {

  // 查找对应的后端分组数据
  const backendGroup = assetStore.userGroups.find((g) => g.id.toString() === group.id.toString());

  console.log("分组编辑", backendGroup, group, assetStore.userGroups)
  if (backendGroup) {
    editingGroup.value = backendGroup;
    showGroupModal.value = true;

    // 如果是从资产类型设置弹窗打开的，关闭资产类型设置弹窗
    if (showAssetTypeSettingsModal.value) {
      showAssetTypeSettingsModal.value = false;
    }
  }
};

// 确认删除分组 - 显示自定义确认对话框
const confirmDeleteGroup = (groupId: string | number) => {
  console.log("确认要删除分组吗?");
  groupToDelete.value = groupId;
  showDeleteConfirm.value = true;
};

// 处理确认删除
const handleDeleteConfirm = () => {
  if (groupToDelete.value !== null) {
    deleteGroup(groupToDelete.value);
  }
  showDeleteConfirm.value = false;
  groupToDelete.value = null;
};

// 取消删除
const cancelDeleteConfirm = () => {
  showDeleteConfirm.value = false;
  groupToDelete.value = null;
};

// 删除分组
const deleteGroup = async (groupId: string | number) => {
  try {
    // 将字符串 ID 转换为数字
    const numericId = typeof groupId === 'string' ? parseInt(groupId) : groupId;
    if (isNaN(numericId)) {
      console.error('Invalid group ID:', groupId);
      return;
    }

    // 调用 store 方法删除分组
    await assetStore.deleteUserGroup(numericId);

    // 从展开列表中移除
    if (typeof groupId === 'string') {
      const expandedIndex = expandedGroups.value.indexOf(groupId);
      if (expandedIndex !== -1) {
        expandedGroups.value.splice(expandedIndex, 1);
      }
    }
  } catch (err) {
    console.error('Failed to delete group:', err);
    alert('删除分组失败');
  }
};

// 打开持仓编辑弹窗
const openPositionEditModal = (asset: WatchlistItem) => {
  selectedAsset.value = asset;
  showPositionEditModal.value = true;

  // 如果是从资产类型设置弹窗打开的，关闭资产类型设置弹窗
  if (showAssetTypeSettingsModal.value) {
    showAssetTypeSettingsModal.value = false;
  }
};

// 关闭持仓编辑弹窗
const closePositionEditModal = () => {
  showPositionEditModal.value = false;
  selectedAsset.value = null;

  // 如果是从资产类型设置弹窗打开的，重新打开资产类型设置弹窗
  if (selectedAssetTypeId.value) {
    showAssetTypeSettingsModal.value = true;
  }
};

// 处理持仓保存
const handlePositionSaved = (data: { symbol: string, position: any }) => {
  // 这里可以添加保存到后端的逻辑
  console.log('Position saved:', data);

  // 如果是从资产类型设置弹窗打开的，重新打开资产类型设置弹窗
  if (selectedAssetTypeId.value) {
    showAssetTypeSettingsModal.value = true;
  }
};

// 打开资产类型设置弹窗
const openAssetTypeSettings = (assetTypeId: number) => {
  selectedAssetTypeId.value = assetTypeId;
  showAssetTypeSettingsModal.value = true;
};

// 关闭资产类型设置弹窗
const closeAssetTypeSettingsModal = () => {
  showAssetTypeSettingsModal.value = false;
  selectedAssetTypeId.value = null;
};

// 打开持仓设置弹窗
const openPositionSettingsModal = () => {
  showPositionModal.value = true;
};

// 开始水平调整大小（左侧上下拖动）
const startResizeHorizontal = (e: MouseEvent) => {
  const isResizingHorizontal = ref(false);
  isResizingHorizontal.value = true;

  const handleMouseMove = (e: MouseEvent) => {
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
provide('positions', assetStore.positions);
const getData = async () => {
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
}
// 初始化
onBeforeMount(() => {
  getData();
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
    background-color: var(--resizerColor);
  }
}
</style>