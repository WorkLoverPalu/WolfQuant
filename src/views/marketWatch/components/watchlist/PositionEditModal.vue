<template>
    <div class="modal-overlay" v-if="show" @click.self="$emit('close')">
        <div class="modal-container">
            <div class="modal-header">
                <h3>编辑持仓信息</h3>
                <button class="close-button" @click="$emit('close')">
                    <XIcon />
                </button>
            </div>
            <div class="modal-body">
                <div class="asset-info">
                    <div class="asset-icon" :class="getSymbolClass(asset.symbol)">
                        {{ getSymbolIcon(asset.symbol) }}
                    </div>
                    <div class="asset-details">
                        <div class="asset-name">{{ asset.name }}</div>
                        <div class="asset-symbol">{{ asset.symbol }}</div>
                    </div>
                    <div class="asset-price">
                        {{ asset.price }} <span class="unit">{{ asset.unit }}</span>
                    </div>
                </div>

                <form @submit.prevent="handleSubmit">
                    <!-- 持仓成本 -->
                    <div class="form-group">
                        <label for="cost">持仓成本 ({{ asset.unit }})</label>
                        <input id="cost" v-model.number="positionData.cost" type="number" step="0.01"
                            class="form-control" required placeholder="请输入每单位持仓成本" />
                    </div>

                    <!-- 持仓数量 -->
                    <div class="form-group">
                        <label for="amount">持仓数量</label>
                        <input id="amount" v-model.number="positionData.amount" type="number" step="0.01"
                            class="form-control" required placeholder="请输入持仓数量" />
                    </div>

                    <!-- 持仓总价值 (只读) -->
                    <div class="form-group">
                        <label>持仓总价值 ({{ asset.unit }})</label>
                        <div class="readonly-value">
                            {{ formatNumber(calculateTotalValue()) }}
                            <span class="profit" :class="calculateProfit().profitRate >= 0 ? 'positive' : 'negative'">
                                {{ calculateProfit().profitRate >= 0 ? '+' : '' }}{{
                                calculateProfit().profitRate.toFixed(2) }}%
                            </span>
                        </div>
                    </div>

                    <!-- 定投计划 -->
                    <div class="form-group">
                        <label>定投计划</label>
                        <div class="investment-plan">
                            <div class="plan-type">
                                <label class="radio-label">
                                    <input type="radio" v-model="positionData.investmentType" value="none" />
                                    <span>不定投</span>
                                </label>
                                <label class="radio-label">
                                    <input type="radio" v-model="positionData.investmentType" value="daily" />
                                    <span>每日</span>
                                </label>
                                <label class="radio-label">
                                    <input type="radio" v-model="positionData.investmentType" value="weekly" />
                                    <span>每周</span>
                                </label>
                                <label class="radio-label">
                                    <input type="radio" v-model="positionData.investmentType" value="monthly" />
                                    <span>每月</span>
                                </label>
                            </div>

                            <!-- 每周选择周几 -->
                            <div class="plan-details" v-if="positionData.investmentType === 'weekly'">
                                <label for="dayOfWeek">选择周几</label>
                                <select id="dayOfWeek" v-model="positionData.dayOfWeek" class="form-control">
                                    <option value="1">周一</option>
                                    <option value="2">周二</option>
                                    <option value="3">周三</option>
                                    <option value="4">周四</option>
                                    <option value="5">周五</option>
                                    <option value="6">周六</option>
                                    <option value="0">周日</option>
                                </select>
                            </div>

                            <!-- 每月选择日期 -->
                            <div class="plan-details" v-if="positionData.investmentType === 'monthly'">
                                <label for="dayOfMonth">选择日期</label>
                                <select id="dayOfMonth" v-model="positionData.dayOfMonth" class="form-control">
                                    <option v-for="day in 31" :key="day" :value="day">{{ day }}日</option>
                                </select>
                            </div>

                            <!-- 定投金额 -->
                            <div class="plan-details" v-if="positionData.investmentType !== 'none'">
                                <label for="investmentAmount">定投金额 ({{ asset.unit }})</label>
                                <input id="investmentAmount" v-model.number="positionData.investmentAmount"
                                    type="number" step="0.01" class="form-control" required placeholder="请输入定投金额" />
                            </div>
                        </div>
                    </div>

                    <div class="form-actions">
                        <button type="button" class="cancel-button" @click="$emit('close')">取消</button>
                        <button type="submit" class="submit-button" :disabled="loading">
                            <LoaderIcon v-if="loading" class="spinner" />
                            保存
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { XIcon, LoaderIcon } from 'lucide-vue-next';
import { useAssetStore, WatchlistItem } from '../../../../stores/assetStore';
import { useInvestmentPlanStore } from '../../../../stores/investmentPlan';
import { useUserStore } from '../../../../stores/userStore';

const props = defineProps<{
    show: boolean;
    asset: WatchlistItem;
}>();

const emit = defineEmits(['close', 'saved']);

const assetStore = useAssetStore();
const investmentPlanStore = useInvestmentPlanStore();
const userStore = useUserStore();
const loading = ref(false);

// 持仓数据
const positionData = ref({
    cost: 0,
    amount: 0,
    investmentType: 'none',
    dayOfWeek: 1,
    dayOfMonth: 1,
    investmentAmount: 0
});

// 获取资产ID
const getAssetId = computed(() => {
    const asset = assetStore.userAssets.find(a => a.code === props.asset.symbol);
    return asset ? asset.id : null;
});

// 获取现有定投计划
const existingPlan = computed(() => {
    if (!getAssetId.value) return null;
    const plans = investmentPlanStore.getPlansByAssetCode(props.asset.symbol);
    return plans.length > 0 ? plans[0] : null;
});

// 初始化持仓数据
onMounted(async () => {
    const position = assetStore.positions[props.asset.symbol];
    
    // 如果没有持仓数据，尝试从资产中获取
    if (!position) {
        const asset = assetStore.userAssets.find(a => a.code === props.asset.symbol);
        if (asset && asset.position_amount !== null && asset.position_cost !== null) {
            positionData.value = {
                cost: asset.position_cost,
                amount: asset.position_amount,
                investmentType: 'none',
                dayOfWeek: 1,
                dayOfMonth: 1,
                investmentAmount: 0
            };
        }
    } else {
        positionData.value = {
            cost: position.cost || 0,
            amount: position.amount || 0,
            investmentType: position.investmentType || 'none',
            dayOfWeek: position.dayOfWeek || 1,
            dayOfMonth: position.dayOfMonth || 1,
            investmentAmount: position.investmentAmount || 0
        };
    }
    
    // 加载定投计划
    try {
        if (getAssetId.value) {
            await investmentPlanStore.getUserInvestmentPlans(userStore.user?.id, getAssetId.value);
        }
    } catch (err) {
        console.error('Failed to load investment plans:', err);
    }
});

// 计算总价值
const calculateTotalValue = () => {
    const price = parseFloat(props.asset.price.replace(/,/g, ''));
    return positionData.value.amount * price;
};

// 计算盈亏
const calculateProfit = () => {
    const totalCost = positionData.value.cost * positionData.value.amount;
    const totalValue = calculateTotalValue();
    const profit = totalValue - totalCost;
    const profitRate = totalCost > 0 ? (profit / totalCost) * 100 : 0;

    return {
        profit,
        profitRate
    };
};

// 获取符号图标
const getSymbolIcon = (symbol: string): string => {
    return symbol.charAt(0);
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

// 格式化数字
const formatNumber = (num: number): string => {
    return num.toLocaleString('zh-CN', { maximumFractionDigits: 2 });
};

// 提交表单
const handleSubmit = async () => {
    try {
        loading.value = true;
        
        // 1. 更新资产持仓数据
        const asset = assetStore.userAssets.find(a => a.code === props.asset.symbol);
        if (asset) {
            await assetStore.updateAsset(
                asset.id,
                asset.name,
                asset.group_id,
                asset.current_price || undefined,
                positionData.value.amount,
                positionData.value.cost
            );
        }
        
        // 2. 处理定投计划
        if (positionData.value.investmentType !== 'none') {
            // 如果选择了定投，创建或更新定投计划
            const userId = userStore.user?.id;
            if (!userId || !getAssetId.value) {
                throw new Error('用户ID或资产ID不存在');
            }
            
            const frequency = investmentPlanStore.mapTypeToFrequency(positionData.value.investmentType);
            const dayOfWeek = positionData.value.investmentType === 'weekly' ? parseInt(positionData.value.dayOfWeek.toString()) : null;
            const dayOfMonth = positionData.value.investmentType === 'monthly' ? parseInt(positionData.value.dayOfMonth.toString()) : null;
            
            if (existingPlan.value) {
                // 更新现有计划
                await investmentPlanStore.updateInvestmentPlan({
                    id: existingPlan.value.id,
                    userId,
                    name: `${props.asset.name}定投计划`,
                    frequency,
                    dayOfWeek,
                    dayOfMonth,
                    amount: positionData.value.investmentAmount,
                    isActive: true
                });
            } else {
                // 创建新计划
                await investmentPlanStore.createInvestmentPlan({
                    userId,
                    assetId: getAssetId.value,
                    name: `${props.asset.name}定投计划`,
                    frequency,
                    dayOfWeek,
                    dayOfMonth,
                    amount: positionData.value.investmentAmount
                });
            }
        } else if (existingPlan.value) {
            // 如果选择了不定投，但存在定投计划，则停用或删除定投计划
            const userId = userStore.user?.id;
            if (!userId) {
                throw new Error('用户ID不存在');
            }
            
            // 选择停用而不是删除，以便用户可以重新激活
            await investmentPlanStore.updateInvestmentPlan({
                id: existingPlan.value.id,
                userId,
                name: existingPlan.value.name,
                frequency: existingPlan.value.frequency,
                dayOfWeek: existingPlan.value.day_of_week,
                dayOfMonth: existingPlan.value.day_of_month,
                amount: existingPlan.value.amount,
                isActive: false
            });
        }

        // 3. 更新前端状态
        assetStore.positions[props.asset.symbol] = {
            cost: positionData.value.cost,
            amount: positionData.value.amount,
            investmentType: positionData.value.investmentType === 'none' ? undefined : positionData.value.investmentType,
            dayOfWeek: positionData.value.investmentType === 'weekly' ? positionData.value.dayOfWeek : undefined,
            dayOfMonth: positionData.value.investmentType === 'monthly' ? positionData.value.dayOfMonth : undefined,
            investmentAmount: positionData.value.investmentType !== 'none' ? positionData.value.investmentAmount : undefined
        };

        emit('saved', {
            symbol: props.asset.symbol,
            position: assetStore.positions[props.asset.symbol]
        });
        emit('close');
    } catch (err) {
        console.error('Failed to save position:', err);
        alert('保存失败: ' + (err instanceof Error ? err.message : String(err)));
    } finally {
        loading.value = false;
    }
};
</script>

<style lang="scss" scoped>
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
    background-color: var(--cardBg);
    border-radius: 8px;
    width: 90%;
    max-width: 500px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
    overflow: hidden;
}

.modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid var(--borderColor);

    h3 {
        margin: 0;
        font-size: 18px;
        font-weight: 600;
        color: var(--textColor);
    }

    .close-button {
        background: transparent;
        border: none;
        color: var(--textSecondary);
        cursor: pointer;
        padding: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 4px;

        &:hover {
            background-color: var(--hover-bg);
            color: var(--textColor);
        }

        svg {
            width: 20px;
            height: 20px;
        }
    }
}

.modal-body {
    padding: 20px;
}

.asset-info {
    display: flex;
    align-items: center;
    margin-bottom: 20px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--borderColor);
}

.asset-icon {
    width: 36px;
    height: 36px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 12px;
    font-size: 16px;
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

.asset-details {
    flex: 1;
}

.asset-name {
    font-weight: 500;
    font-size: 16px;
    color: var(--textColor);
}

.asset-symbol {
    font-size: 14px;
    color: var(--textSecondary);
    margin-top: 2px;
}

.asset-price {
    font-weight: 600;
    font-size: 16px;
    color: var(--textColor);
}

.unit {
    font-size: 12px;
    color: var(--textSecondary);
    margin-left: 2px;
}

.form-group {
    margin-bottom: 16px;

    label {
        display: block;
        margin-bottom: 6px;
        font-size: 14px;
        font-weight: 500;
        color: var(--textColor);
    }

    .form-control {
        width: 100%;
        padding: 10px 12px;
        border: 1px solid var(--borderColor);
        border-radius: 4px;
        background-color: var(--inputBg);
        color: var(--textColor);
        font-size: 14px;
        transition: border-color 0.2s;

        &:focus {
            border-color: var(--accentColor);
            outline: none;
        }
    }
}

.readonly-value {
    padding: 10px 12px;
    border: 1px solid var(--borderColor);
    border-radius: 4px;
    background-color: var(--bgColor);
    color: var(--textColor);
    font-size: 14px;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.profit {
    font-size: 13px;
    font-weight: 500;

    &.positive {
        color: var(--positiveColor);
    }

    &.negative {
        color: var(--negativeColor);
    }
}

.investment-plan {
    border: 1px solid var(--borderColor);
    border-radius: 4px;
    padding: 12px;
    background-color: var(--bgColor);
}

.plan-type {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    margin-bottom: 12px;
}

.radio-label {
    display: flex;
    align-items: center;
    cursor: pointer;

    input {
        margin-right: 6px;
    }

    span {
        font-size: 14px;
    }
}

.plan-details {
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px solid var(--borderColor);

    label {
        display: block;
        margin-bottom: 6px;
        font-size: 14px;
        font-weight: 500;
        color: var(--textColor);
    }
}

.form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 24px;

    button {
        padding: 10px 16px;
        border-radius: 4px;
        font-size: 14px;
        font-weight: 500;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 6px;
    }

    .cancel-button {
        background-color: transparent;
        border: 1px solid var(--borderColor);
        color: var(--textColor);

        &:hover {
            background-color: var(--hover-bg);
        }
    }

    .submit-button {
        background-color: var(--accentColor);
        border: none;
        color: white;

        &:hover {
            background-color: var(--accentColorHover);
        }

        &:disabled {
            opacity: 0.7;
            cursor: not-allowed;
        }
    }
}

.spinner {
    animation: spin 1s linear infinite;
}

@keyframes spin {
    from {
        transform: rotate(0deg);
    }

    to {
        transform: rotate(360deg);
    }
}
</style>