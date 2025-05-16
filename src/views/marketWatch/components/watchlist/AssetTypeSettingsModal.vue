<template>
    <div class="modal-overlay" v-if="show" @click.self="$emit('close')">
        <div class="modal-container">
            <div class="modal-header">
                <h3>{{ assetTypeName }}设置</h3>
                <button class="close-button" @click="$emit('close')">
                    <XIcon />
                </button>
            </div>
            <div class="modal-body">
                <div class="tabs">
                    <button class="tab-button" :class="{ active: activeTab === 'groups' }"
                        @click="activeTab = 'groups'">
                        分组管理
                    </button>
                    <button class="tab-button" :class="{ active: activeTab === 'positions' }"
                        @click="activeTab = 'positions'">
                        持仓设置
                    </button>
                </div>

                <!-- 分组管理 -->
                <div v-if="activeTab === 'groups'" class="tab-content">
                    <div class="groups-list">
                        <div v-for="group in filteredGroups" :key="group.id" class="group-item">
                            <div class="group-info">
                                <div class="group-name">{{ group.name }}</div>
                                <div class="group-description">{{ group.description || '无描述' }}</div>
                            </div>
                            <div class="group-actions">
                                <button class="action-button" @click="editGroup(group)" title="编辑分组">
                                    <EditIcon />
                                </button>
                                <button class="action-button" @click="confirmDeleteGroup(group.id)" title="删除分组">
                                    <TrashIcon />
                                </button>
                            </div>
                        </div>

                        <div v-if="filteredGroups.length === 0" class="empty-state">
                            <div class="empty-message">暂无分组</div>
                        </div>
                    </div>

                    <button class="add-button" @click="openAddGroupModal">
                        <PlusIcon />
                        添加新分组
                    </button>
                </div>

                <!-- 持仓设置 -->
                <div v-if="activeTab === 'positions'" class="tab-content">
                    <div class="positions-list">
                        <div v-for="asset in filteredAssets" :key="asset.symbol" class="asset-item">
                            <div class="asset-info">
                                <div class="asset-icon" :class="getSymbolClass(asset.symbol)">
                                    {{ getSymbolIcon(asset.symbol) }}
                                </div>
                                <div class="asset-details">
                                    <div class="asset-name">{{ asset.name }}</div>
                                    <div class="asset-symbol">{{ asset.symbol }}</div>
                                </div>
                            </div>
                            <div class="position-info">
                                <div v-if="hasPosition(asset.symbol)" class="position-value">
                                    <div>持仓: {{ formatNumber(getPosition(asset.symbol).amount) }}</div>
                                    <div class="investment-type">
                                        {{ getInvestmentTypeText(getPosition(asset.symbol).investmentType) }}
                                    </div>
                                </div>
                                <div v-else class="no-position">未设置持仓</div>
                            </div>
                            <button class="edit-button" @click="editPosition(asset)" title="编辑持仓">
                                <EditIcon />
                            </button>
                        </div>

                        <div v-if="filteredAssets.length === 0" class="empty-state">
                            <div class="empty-message">暂无资产</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { XIcon, EditIcon, TrashIcon, PlusIcon } from 'lucide-vue-next';
import { useAssetStore, WatchlistItem, UserGroup } from '../../../../stores/assetStore';

const props = defineProps<{
    show: boolean;
    assetTypeId: number;
}>();

const emit = defineEmits(['close', 'openAddGroup', 'editGroup', 'deleteGroup', 'editPosition']);

const assetStore = useAssetStore();
const activeTab = ref('groups');

// 获取资产类型名称
const assetTypeName = computed(() => {
    const assetType = assetStore.assetTypes.find(type => type.id === props.assetTypeId);
    return assetType ? assetType.description || assetType.name : '资产';
});

// 过滤当前资产类型的分组
const filteredGroups = computed(() => {
    return assetStore.userGroups.filter(group => group.asset_type_id === props.assetTypeId);
});

// 获取当前资产类型下的所有资产
const filteredAssets = computed(() => {
    // 获取当前资产类型下的所有资产
    const assets = assetStore.userAssets.filter(asset => asset.asset_type_id === props.assetTypeId);

    // 转换为 WatchlistItem 格式
    return assets.map(asset => ({
        symbol: asset.code,
        name: asset.name,
        price: asset.current_price.toFixed(2),
        unit: 'USD', // 默认单位
        change: '0.00',
        changePercent: '0.00%',
        volume: '—',
        turnover: '—'
    }));
});

// 检查是否有持仓
const hasPosition = (symbol: string): boolean => {
    return !!assetStore.positions[symbol];
};

// 获取持仓信息
const getPosition = (symbol: string) => {
    return assetStore.positions[symbol] || { cost: 0, amount: 0 };
};

// 获取定投类型文本
const getInvestmentTypeText = (type?: string): string => {
    const typeMap: Record<string, string> = {
        'daily': '每日定投',
        'weekly': '每周定投',
        'monthly': '每月定投'
    };

    return type ? typeMap[type] : '不定投';
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

// 打开添加分组模态框
const openAddGroupModal = () => {
    emit('openAddGroup', props.assetTypeId);
};

// 编辑分组
const editGroup = (group: UserGroup) => {
    emit('editGroup', group);
};

// 确认删除分组
const confirmDeleteGroup = (groupId: number) => {
    emit('deleteGroup', groupId);
};

// 编辑持仓
const editPosition = (asset: WatchlistItem) => {
    emit('editPosition', asset);
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
    max-width: 600px;
    max-height: 80vh;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
    overflow: hidden;
    display: flex;
    flex-direction: column;
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
    overflow-y: auto;
    flex: 1;
}

.tabs {
    display: flex;
    border-bottom: 1px solid var(--borderColor);
    margin-bottom: 20px;
}

.tab-button {
    padding: 10px 16px;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--textSecondary);
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;

    &.active {
        color: var(--accentColor);
        border-bottom-color: var(--accentColor);
    }

    &:hover:not(.active) {
        color: var(--textColor);
    }
}

.tab-content {
    max-height: 400px;
    overflow-y: auto;
}

.groups-list,
.positions-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.group-item,
.asset-item {
    display: flex;
    align-items: center;
    padding: 12px;
    border-radius: 6px;
    background-color: var(--bgColor);
    border: 1px solid var(--borderColor);
}

.group-info {
    flex: 1;
}

.group-name {
    font-weight: 500;
    font-size: 14px;
    color: var(--textColor);
}

.group-description {
    font-size: 12px;
    color: var(--textSecondary);
    margin-top: 2px;
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
    color: var(--textSecondary);
    cursor: pointer;

    &:hover {
        background-color: var(--hover-bg);
        color: var(--textColor);
    }

    svg {
        width: 16px;
        height: 16px;
    }
}

.asset-info {
    display: flex;
    align-items: center;
    flex: 1;
}

.asset-icon {
    width: 28px;
    height: 28px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 10px;
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

.asset-details {
    display: flex;
    flex-direction: column;
}

.asset-name {
    font-weight: 500;
    font-size: 14px;
    color: var(--textColor);
}

.asset-symbol {
    font-size: 12px;
    color: var(--textSecondary);
    margin-top: 2px;
}

.position-info {
    margin-right: 12px;
    text-align: right;
}

.position-value {
    font-size: 13px;
    color: var(--textColor);
}

.investment-type {
    font-size: 12px;
    color: var(--accentColor);
    margin-top: 2px;
}

.no-position {
    font-size: 13px;
    color: var(--textSecondary);
}

.edit-button {
    width: 28px;
    height: 28px;
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
        width: 16px;
        height: 16px;
    }
}

.empty-state {
    padding: 24px 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.empty-message {
    color: var(--textSecondary);
    font-size: 14px;
}

.add-button {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    width: 100%;
    padding: 10px;
    margin-top: 16px;
    background-color: var(--accentColor);
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;

    &:hover {
        background-color: var(--accentColorHover);
    }

    svg {
        width: 16px;
        height: 16px;
    }
}
</style>