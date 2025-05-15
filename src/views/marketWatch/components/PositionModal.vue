<template>
  <div class="modal-overlay" @click="closeModal">
    <div class="modal-container position-modal" @click.stop>
      <div class="modal-header">
        <h3>持仓金额设置</h3>
        <button class="close-button" @click="closeModal">
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
                <input type="number" class="position-input" placeholder="输入成本价"
                  v-model="getPositionInput(item.symbol).cost" />
              </div>
              <div class="input-group">
                <label>持仓金额</label>
                <input type="number" class="position-input" placeholder="输入金额"
                  v-model="getPositionInput(item.symbol).amount" />
              </div>
              <div class="input-group">
                <label>定投类型</label>
                <select class="position-input" v-model="getPositionInput(item.symbol).investmentType">
                  <option value="daily">每日</option>
                  <option value="weekly">每周</option>
                  <option value="biweekly">每两周</option>
                  <option value="monthly">每月</option>
                </select>
              </div>
              <div v-if="getPositionInput(item.symbol).investmentType === 'weekly'" class="input-group">
                <label>选择周几</label>
                <select class="position-input" v-model="getPositionInput(item.symbol).dayOfWeek">
                  <option value="1">周一</option>
                  <option value="2">周二</option>
                  <option value="3">周三</option>
                  <option value="4">周四</option>
                  <option value="5">周五</option>
                  <option value="6">周六</option>
                  <option value="7">周日</option>
                </select>
              </div>
              <div v-if="getPositionInput(item.symbol).investmentType === 'monthly'" class="input-group">
                <label>选择日期</label>
                <select class="position-input" v-model="getPositionInput(item.symbol).dayOfMonth">
                  <option v-for="day in 31" :key="day" :value="day">
                    {{ day }}日
                  </option>
                </select>
              </div>
              <div class="input-group">
                <label>定投金额</label>
                <input type="number" class="position-input" placeholder="输入金额"
                  v-model="getPositionInput(item.symbol).investmentAmount" />
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="cancel-button" @click="closeModal">取消</button>
        <button class="confirm-button" @click="savePositions">保存</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, computed } from 'vue';
import { XIcon } from 'lucide-vue-next';

// 注入全局状态
const showPositionModal = inject('showPositionModal');
const activeCategory = inject<string>('activeCategory');
const groups = inject('groups');
const positions = inject('positions');

// 持仓输入临时存储
interface PositionInput {
  cost: string;
  amount: string;
  investmentType: string;
  investmentAmount: string;
  dayOfWeek?: string;
  dayOfMonth?: number;
}

const positionInputs = ref<Record<string, PositionInput>>({});

// 根据当前分类过滤分组
const filteredGroups = computed(() => {
  if (activeCategory.value === 'all') {
    return groups.value;
  }
  return groups.value.filter(group => group.category === activeCategory.value);
});

// 获取持仓输入
const getPositionInput = (symbol:any) => {
  if (!positionInputs.value[symbol]) {
    const position = positions.value[symbol] || { cost: 0, amount: 0 };
    positionInputs.value[symbol] = {
      cost: position.cost ? position.cost.toString() : '',
      amount: position.amount ? position.amount.toString() : '',
      investmentType: 'monthly',
      investmentAmount: '0'
    };
  }
  return positionInputs.value[symbol];
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
    '518880': 'symbol-fund'
  };

  return symbolMap[symbol] || 'symbol-default';
};

// 关闭模态框
const closeModal = () => {
  showPositionModal.value = false;
  positionInputs.value = {};
};

// 保存持仓设置
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

  closeModal();
};
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