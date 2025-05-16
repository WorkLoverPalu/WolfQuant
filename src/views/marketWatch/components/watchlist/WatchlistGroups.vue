<template>
  <div class="watchlist-groups">
    <div v-for="(group, index) in groups" :key="group.id" class="watchlist-group"
      :class="{ 'is-dragging': draggedGroup === group.id }" draggable="true"
      @dragstart="$emit('handleDragStart', $event, group.id)"
      @dragover.prevent="$emit('handleDragOver', $event, group.id)" @dragend="$emit('handleDragEnd')"
      @drop="$emit('handleDrop', $event, group.id)">
      <div class="group-header">
        <div class="group-title" @click="$emit('toggleGroup', group.id)">
          <ChevronDownIcon v-if="expandedGroups.includes(group.id)" />
          <ChevronRightIcon v-else />
          {{ group.name }}

          <div v-if="getGroupPosition(group.id).total > 0" class="group-position">
            <span class="position-amount">¥{{ formatNumber(getGroupPosition(group.id).total) }}</span>
            <span class="position-profit" :class="getGroupPosition(group.id).profitRate >= 0 ? 'positive' : 'negative'">
              {{ getGroupPosition(group.id).profitRate >= 0 ? '+' : '' }}{{
                getGroupPosition(group.id).profitRate.toFixed(2) }}%
            </span>
          </div>
        </div>
        <div class="group-actions">
          <button class="action-button" @click="$emit('openAddSymbolModal', group.id)" title="添加商品">
            <PlusIcon />
          </button>
          <button class="action-button" @click="$emit('editGroup', group)" title="编辑分组">
            <EditIcon />
          </button>
          <button class="action-button" @click="$emit('deleteGroup', group.id)" title="删除分组">
            <TrashIcon />
          </button>
        </div>
      </div>

      <div v-if="expandedGroups.includes(group.id)" class="group-content">
        <!-- 走势图视图 -->
        <div v-if="showChartView" class="chart-view">
          <div v-for="item in group.items" :key="item.symbol" class="chart-item" @click="$emit('selectSymbol', item)">
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
              <div class="price-info" :class="getChangeClass(item.change)">
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
            <div v-for="item in group.items" :key="item.symbol" class="list-item" @click="$emit('selectSymbol', item)">
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
              <div class="column change-column" :class="getChangeClass(item.change)">
                {{ item.change }}
              </div>
              <div class="column change-percent-column" :class="getChangeClass(item.changePercent)">
                {{ item.changePercent }}
              </div>
              <div class="column position-column">
                {{ getItemPosition(item.symbol) ? '¥' + formatNumber(getItemPosition(item.symbol).amount) : '—' }}
              </div>
              <div class="column profit-column" :class="getItemPosition(item.symbol) && getItemPosition(item.symbol).profit > 0 ? 'positive' :
                getItemPosition(item.symbol) && getItemPosition(item.symbol).profit < 0 ? 'negative' : ''">
                {{ getItemPosition(item.symbol) ? (getItemPosition(item.symbol).profit > 0 ? '+' : '') +
                  formatNumber(getItemPosition(item.symbol).profit) : '—' }}
              </div>
            </div>

            <!-- 空状态 -->
            <div v-if="group.items.length === 0" class="empty-state">
              <div class="empty-message">该分组暂无数据</div>
              <button class="add-item-button" @click="$emit('openAddSymbolModal', group.id)">
                <PlusIcon />
                添加商品
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, computed } from 'vue';
import {
  ChevronDownIcon,
  ChevronRightIcon,
  PlusIcon,
  EditIcon,
  TrashIcon
} from 'lucide-vue-next';
import { useAssetStore } from '../../../../stores/assetStore';

// 定义类型接口
interface WatchlistItem {
  symbol: string;
  name: string;
  price: string;
  unit: string;
  change: string;
  changePercent: string;
  volume?: string;
  turnover?: string;
}

interface WatchlistGroup {
  id: string;
  name: string;
  category: string;
  description: string;
  items: WatchlistItem[];
}

interface Position {
  cost: number;
  amount: number;
}

interface GroupPosition {
  total: number;
  profit: number;
  profitRate: number;
}

interface ItemPosition {
  cost: number;
  amount: number;
  profit: number;
  profitRate: number;
}

// 接收父组件传递的属性
const props = defineProps<{
  groups: WatchlistGroup[];
  expandedGroups: string[];
  showChartView: boolean;
  draggedGroup: string | null;
}>();

// 定义事件
const emit = defineEmits([
  'toggleGroup',
  'selectSymbol',
  'openAddSymbolModal',
  'editGroup',
  'deleteGroup',
  'handleDragStart',
  'handleDragOver',
  'handleDragEnd',
  'handleDrop'
]);

// 使用 store
const assetStore = useAssetStore();

// 注入数据
const positions = inject<Record<string, Position>>('positions', {});

// 获取符号图标
const getSymbolIcon = (symbol: string): string => {
  const firstChar = symbol.charAt(0);
  return firstChar;
};

// 获取符号类名
const getSymbolClass = (symbol: string): string => {
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
const getChangeClass = (change: string): string => {
  if (change.startsWith('-')) {
    return 'negative';
  } else if (change.startsWith('+') || parseFloat(change.replace(',', '')) > 0) {
    return 'positive';
  }
  return '';
};

// 格式化数字
const formatNumber = (num: number): string => {
  return num.toLocaleString('zh-CN', { maximumFractionDigits: 2 });
};

// 获取单个商品的持仓信息
const getItemPosition = (symbol: string): ItemPosition | null => {
  const position = positions[symbol];
  if (!position || !position.amount) return null;

  // 查找商品当前价格
  let currentPrice = 0;
  for (const group of props.groups) {
    const item = group.items.find(i => i.symbol === symbol);
    if (item) {
      currentPrice = parseFloat(item.price.replace(',', ''));
      break;
    }
  }

  if (!currentPrice) return null;

  const cost = position.cost * position.amount;
  const currentValue = position.amount * currentPrice;
  const profit = currentValue - cost;

  return {
    cost,
    amount: position.amount,
    profit,
    profitRate: (profit / cost) * 100
  };
};

// 获取分组的持仓信息
const getGroupPosition = (groupId: string): GroupPosition => {
  const group = props.groups.find(g => g.id === groupId);
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
</script>

<style lang="scss" scoped>
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
    background: var(--scrollbarThumb);
    border-radius: 3px;
  }
}

.watchlist-group {
  margin-bottom: 8px;
  border-radius: 6px;
  background-color: var(--cardBg);
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
  background-color: var(--headerBg);
  border-bottom: 1px solid var(--borderColor);
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
  color: var(--textSecondary);
}

.position-profit {
  margin-left: 6px;

  &.positive {
    color: var(--positiveColor);
  }

  &.negative {
    color: var(--negativeColor);
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
  color: var(--textSecondary);
  cursor: pointer;

  &:hover {
    background-color: var(--hover-bg);
    color: var(--textColor);
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
  border-bottom: 1px solid var(--borderColor);
  font-size: 11px;
  color: var(--textSecondary);
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
    background: var(--scrollbarThumb);
    border-radius: 2px;
  }
}

.list-item {
  display: flex;
  padding: 8px 12px;
  border-bottom: 1px solid var(--borderColor);
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

.price-column,
.change-column,
.change-percent-column {
  width: 60px;
  text-align: right;
}

.position-column,
.profit-column {
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
  color: var(--textSecondary);
  margin-top: 1px;
}

/* 价格单位 */
.unit {
  font-size: 11px;
  color: var(--textSecondary);
  margin-left: 2px;
}

/* 涨跌颜色 */
.positive {
  color: var(--positiveColor);
}

.negative {
  color: var(--negativeColor);
}

/* 走势图视图 */
.chart-view {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 8px;
  padding: 8px;
}

.chart-item {
  background-color: var(--cardBg);
  border-radius: 4px;
  border: 1px solid var(--borderColor);
  overflow: hidden;
  cursor: pointer;

  &:hover {
    border-color: var(--accentColor);
  }
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px;
  border-bottom: 1px solid var(--borderColor);
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
  background-image: linear-gradient(to right, var(--chartGridColor) 1px, transparent 1px),
    linear-gradient(to bottom, var(--chartGridColor) 1px, transparent 1px);
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
  color: var(--textSecondary);
  margin-bottom: 12px;
  font-size: 13px;
}

.add-item-button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background-color: var(--buttonBg);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;

  &:hover {
    background-color: var(--buttonHoverBg);
  }

  svg {
    width: 14px;
    height: 14px;
  }
  
}
</style>