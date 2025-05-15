<template>
    <div class="modal-overlay" @click="closeModal">
        <div class="modal-container" @click.stop>
            <div class="modal-header">
                <h3>添加商品代码</h3>
                <button class="close-button" @click="closeModal">
                    <XIcon />
                </button>
            </div>

            <div class="modal-search">
                <SearchIcon class="search-icon" />
                <input type="text" class="search-input" placeholder="输入代码或名称搜索" v-model="searchQuery"
                    @input="searchSymbols" />
            </div>

            <div class="modal-tabs">
                <button v-for="category in assetStore.categories" :key="category.id" class="modal-tab"
                    :class="{ active: modalActiveCategory === category.id }"
                    @click="setModalActiveCategory(category.id)">
                    {{ category.name }}
                </button>
            </div>

            <div class="modal-results">
                <div v-if="loading" class="loading-results">
                    <div class="loading-spinner"></div>
                    <div>加载中...</div>
                </div>

                <template v-else>
                    <div v-for="(result, index) in searchResults" :key="index" class="result-item">
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
                        <button class="add-to-watchlist-button" @click="addSymbolToWatchlist(result)"
                            :disabled="addingAsset">
                            {{ addingAsset === result.symbol ? '添加中...' : '添加到自选表' }}
                        </button>
                    </div>

                    <div v-if="searchResults.length === 0" class="empty-results">
                        没有找到匹配的结果
                    </div>
                </template>
            </div>

            <div v-if="targetGroupId" class="modal-footer">
                <div class="target-group-info">
                    添加到分组:
                    <span class="target-group-name">
                        {{ getGroupName(targetGroupId) }}
                    </span>
                    <button class="change-group-button" @click="showGroupSelector = true">
                        更改
                    </button>
                </div>
            </div>

            <!-- 分组选择器 -->
            <div v-if="showGroupSelector" class="group-selector-overlay" @click="showGroupSelector = false">
                <div class="group-selector" @click.stop>
                    <div class="group-selector-header">
                        <h4>选择分组</h4>
                        <button class="close-button" @click="showGroupSelector = false">
                            <XIcon />
                        </button>
                    </div>
                    <div class="group-selector-content">
                        <div v-for="group in assetStore.userGroups" :key="group.id" class="group-selector-item"
                            @click="selectTargetGroup(group.id.toString())">
                            {{ group.name }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, defineProps, defineEmits } from 'vue';
import { XIcon, SearchIcon } from 'lucide-vue-next';
import { useAssetStore } from '../../../stores/assetStore';
import { useUserStore } from '../../../stores/userStore';
import type { AssetType, UserGroup } from '../../../stores/assetStore';

const props = defineProps<{
    show: boolean;
    initialGroupId?: string;
}>();

const emit = defineEmits<{
    (e: 'close'): void;
    (e: 'added'): void;
}>();

// 使用 store
const assetStore = useAssetStore();
const userStore = useUserStore();



// 搜索相关状态
const searchQuery = ref('');
const searchResults = ref<any[]>([]);
const modalActiveCategory = ref(assetStore.activeCategory || 'all');
const targetGroupId = ref<string | null>(props.initialGroupId || null);
const showGroupSelector = ref(false);
const loading = ref(false);
const addingAsset = ref<string | null>(null);
const error = ref<string | null>(null);

// 关闭模态框
const closeModal = () => {
    emit('close');
};

// 设置模态框分类
const setModalActiveCategory = (category: string) => {
    modalActiveCategory.value = category;
    searchSymbols();
};

// 获取分组名称
const getGroupName = (groupId: string) => {
    const group = assetStore.userGroups.find(g => g.id.toString() === groupId);
    return group ? group.name : '未知分组';
};

// 选择目标分组
const selectTargetGroup = (groupId: string) => {
    targetGroupId.value = groupId;
    showGroupSelector.value = false;
};

// 将资产类型代码映射到前端分类
const mapAssetTypeToCategory = (assetTypeCode: string): string => {
    const mapping: Record<string, string> = {
        'FUND': 'fund',
        'STOCK': 'stock',
        'GOLD': 'gold',
        'CRYPTO': 'crypto'
    };

    return mapping[assetTypeCode.toUpperCase()] || 'other';
};

// 将前端分类映射到资产类型ID
const getCategoryAssetTypeId = (category: string): number | null => {
    if (category === 'all') return null;

    const assetType = assetStore.assetTypes.find(type => {
        const typeCategory = mapAssetTypeToCategory(type.code);
        return typeCategory === category;
    });

    return assetType ? assetType.id : null;
};

// 搜索商品
const searchSymbols = async () => {
    loading.value = true;
    error.value = null;

    try {
        // 如果有搜索词，使用模拟数据（实际项目中可以替换为API调用）
        if (searchQuery.value) {
            // 模拟搜索结果
            const mockResults = [
                { symbol: '518880', name: '黄金基金', meta: 'fund etf SSE', category: 'fund', assetTypeId: getAssetTypeIdByCategory('fund') },
                { symbol: '159934', name: '黄金ETF', meta: 'fund etf SZSE', category: 'fund', assetTypeId: getAssetTypeIdByCategory('fund') },
                { symbol: 'XAUUSD', name: '黄金/美元', meta: 'spot gold', category: 'gold', assetTypeId: getAssetTypeIdByCategory('gold') },
                { symbol: 'GC', name: '黄金期货', meta: 'futures COMEX', category: 'gold', assetTypeId: getAssetTypeIdByCategory('gold') }
            ];

            // 根据搜索词和当前分类过滤
            searchResults.value = mockResults.filter(item => {
                const matchesSearch =
                    item.symbol.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                    item.name.toLowerCase().includes(searchQuery.value.toLowerCase());

                const matchesCategory = modalActiveCategory.value === 'all' ||
                    item.category === modalActiveCategory.value;

                return matchesSearch && matchesCategory;
            });
        } else {
            // 如果没有搜索词，从现有资产中获取建议
            const suggestions = [];

            // 从现有的市场数据中获取建议
            for (const group of assetStore.groups) {
                if (modalActiveCategory.value === 'all' || group.category === modalActiveCategory.value) {
                    for (const item of group.items) {
                        suggestions.push({
                            symbol: item.symbol,
                            name: item.name,
                            meta: `${group.category} ${item.unit}`,
                            category: group.category,
                            assetTypeId: getAssetTypeIdByCategory(group.category)
                        });
                    }
                }
            }

            searchResults.value = suggestions;
        }
    } catch (err) {
        console.error('Search failed:', err);
        error.value = '搜索失败';
    } finally {
        loading.value = false;
    }
};

// 根据分类获取资产类型ID
const getAssetTypeIdByCategory = (category: string): number => {
    const assetType = assetStore.assetTypes.find(type => {
        const typeCategory = mapAssetTypeToCategory(type.code);
        return typeCategory === category;
    });

    return assetType ? assetType.id : 0;
};

// 添加商品到自选表
const addSymbolToWatchlist = async (symbol: any) => {
    if (!targetGroupId.value) {
        // 如果没有指定目标分组，尝试找到匹配的分类分组
        const matchingGroup = assetStore.userGroups.find(group => {
            const assetType = assetStore.assetTypes.find(type => type.id === group.asset_type_id);
            if (!assetType) return false;

            const groupCategory = mapAssetTypeToCategory(assetType.code);
            return groupCategory === symbol.category;
        });

        if (matchingGroup) {
            targetGroupId.value = matchingGroup.id.toString();
        } else {
            // 如果没有匹配的分组，显示分组选择器
            showGroupSelector.value = true;
            return;
        }
    }

    // 检查是否已存在
    const groupAssets = assetStore.userAssets.filter(asset =>
        asset.group_id === parseInt(targetGroupId.value!) &&
        asset.code === symbol.symbol
    );

    if (groupAssets.length > 0) {
        alert('该资产已存在于所选分组中');
        return;
    }

    // 添加资产
    addingAsset.value = symbol.symbol;

    try {
        const assetTypeId = symbol.assetTypeId || getAssetTypeIdByCategory(symbol.category);

        if (!assetTypeId) {
            throw new Error('无法确定资产类型');
        }

        // 创建新资产
        await assetStore.createAsset(
            parseInt(targetGroupId.value!),
            assetTypeId,
            symbol.symbol,
            symbol.name,
            parseFloat(symbol.price || '0')
        );

        // 刷新资产列表
        await assetStore.fetchUserAssets(undefined, parseInt(targetGroupId.value!));

        emit('added');
        closeModal();
    } catch (err) {
        console.error('Failed to add asset:', err);
        alert('添加资产失败');
    } finally {
        addingAsset.value = null;
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
        '518880': 'symbol-fund',
        '159934': 'symbol-fund',
        'GC': 'symbol-gold'
    };

    return symbolMap[symbol] || 'symbol-default';
};

// 初始化
onMounted(async () => {
    // 加载资产类型
    if (assetStore.assetTypes.length === 0) {
        try {
            await assetStore.fetchAssetTypes();
        } catch (err) {
            console.error('Failed to load asset types:', err);
        }
    }

    // 加载用户分组
    if (assetStore.userGroups.length === 0) {
        try {
            await assetStore.fetchUserGroups();
        } catch (err) {
            console.error('Failed to load user groups:', err);
        }
    }

    // 如果没有指定目标分组，但有激活分类，尝试找到匹配的分组
    if (!targetGroupId.value && modalActiveCategory.value !== 'all') {
        const matchingGroup = assetStore.userGroups.find(group => {
            const assetType = assetStore.assetTypes.find(type => type.id === group.asset_type_id);
            if (!assetType) return false;

            const groupCategory = mapAssetTypeToCategory(assetType.code);
            return groupCategory === modalActiveCategory.value;
        });

        if (matchingGroup) {
            targetGroupId.value = matchingGroup.id.toString();
        }
    }

    searchSymbols();
});
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
    background-color: var(--modalBg);
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
    border-bottom: 1px solid var(--borderColor);

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
        color: var(--textSecondary);
        cursor: pointer;

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

.modal-search {
    position: relative;
    padding: 16px;
    border-bottom: 1px solid var(--borderColor);
}

.search-icon {
    position: absolute;
    left: 28px;
    top: 50%;
    transform: translateY(-50%);
    width: 20px;
    height: 20px;
    color: var(--textSecondary);
}

.search-input {
    width: 100%;
    height: 40px;
    padding: 0 16px 0 40px;
    background-color: var( --inputBg);
    border: 1px solid var(--borderColor);
    border-radius: 4px;
    color: var(--textColor);
    font-size: 14px;

    &:focus {
        outline: none;
        border-color: var(--accentColor);
    }

    &::placeholder {
        color: var(--textSecondary);
    }
}

.modal-tabs {
    display: flex;
    overflow-x: auto;
    border-bottom: 1px solid var(--borderColor);

    &::-webkit-scrollbar {
        height: 0;
    }
}

.modal-tab {
    padding: 12px 16px;
    background: transparent;
    border: none;
    color: var(--textSecondary);
    font-size: 14px;
    cursor: pointer;
    white-space: nowrap;

    &:hover {
        color: var(--textColor);
    }

    &.active {
        color: var(--accentColor);
        position: relative;

        &::after {
            content: '';
            position: absolute;
            bottom: -1px;
            left: 0;
            width: 100%;
            height: 2px;
            background-color: var(--accentColor);
        }
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
        background: var(--scrollbarThumb);
        border-radius: 2px;
    }
}

.loading-results {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 32px 16px;
    color: var(--textSecondary);

    .loading-spinner {
        width: 24px;
        height: 24px;
        border: 2px solid var(--textSecondary);
        border-top-color: transparent;
        border-radius: 50%;
        margin-bottom: 12px;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
}

.result-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--borderColor);

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

.result-details {
    display: flex;
    flex-direction: column;
}

.result-code {
    font-size: 14px;
    font-weight: 500;

    .result-highlight {
        margin-left: 8px;
        color: var(--textSecondary);
    }
}

.result-meta {
    font-size: 12px;
    color: var(--textSecondary);
    margin-top: 2px;
}

.add-to-watchlist-button {
    padding: 6px 12px;
    background-color: var(--buttonBg);
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 12px;
    cursor: pointer;

    &:hover:not(:disabled) {
        background-color: var( --buttonHoverBg);
    }

    &:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
}

.empty-results {
    padding: 32px 16px;
    text-align: center;
    color: var(--textSecondary);
}

.modal-footer {
    padding: 12px 16px;
    border-top: 1px solid var(--borderColor);
}

.target-group-info {
    display: flex;
    align-items: center;
    font-size: 14px;
    color: var(--textSecondary);

    .target-group-name {
        margin: 0 8px;
        font-weight: 500;
        color: var(--textColor);
    }

    .change-group-button {
        padding: 4px 8px;
        background-color: transparent;
        border: 1px solid var(--borderColor);
        border-radius: 4px;
        color: var(--textColor);
        font-size: 12px;
        cursor: pointer;

        &:hover {
            background-color: var(--hover-bg);
        }
    }
}

.group-selector-overlay {
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

.group-selector {
    width: 90%;
    max-width: 400px;
    max-height: 80vh;
    background-color: var(--modalBg);
    border-radius: 8px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
}

.group-selector-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--borderColor);

    h4 {
        margin: 0;
        font-size: 16px;
        font-weight: 500;
    }
}

.group-selector-content {
    max-height: 300px;
    overflow-y: auto;
}

.group-selector-item {
    padding: 12px 16px;
    border-bottom: 1px solid var(--borderColor);
    cursor: pointer;

    &:last-child {
        border-bottom: none;
    }

    &:hover {
        background-color: var(--hover-bg);
    }
}
</style>