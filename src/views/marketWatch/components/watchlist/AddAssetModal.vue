<template>
    <div class="modal-overlay" v-if="show" @click.self="$emit('close')">
        <div class="modal-container">
            <div class="modal-header">
                <h3>添加资产</h3>
                <button class="close-button" @click="$emit('close')">
                    <XIcon />
                </button>
            </div>
            <div class="modal-body">
                <div class="search-container">
                    <input type="text" v-model="searchQuery" class="search-input" placeholder="搜索资产代码或名称..."
                        @input="handleSearch" />
                    <SearchIcon class="search-icon" />
                </div>

                <div v-if="loading" class="loading-container">
                    <LoaderIcon class="spinner" />
                    <span>加载中...</span>
                </div>

                <div v-else-if="error" class="error-container">
                    <AlertCircleIcon />
                    <span>{{ error }}</span>
                </div>

                <div v-else-if="searchResults.length === 0 && searchQuery" class="empty-results">
                    <div class="empty-message">
                        <SearchXIcon />
                        <span>未找到匹配的资产</span>
                    </div>
                    <div class="create-new">
                        <p>创建新资产:</p>
                        <form @submit.prevent="handleCreateAsset">
                            <div class="form-group">
                                <label for="assetCode">资产代码</label>
                                <input id="assetCode" v-model="newAsset.code" class="form-control" required
                                    placeholder="例如: AAPL" />
                            </div>
                            <div class="form-group">
                                <label for="assetName">资产名称</label>
                                <input id="assetName" v-model="newAsset.name" class="form-control" required
                                    placeholder="例如: 苹果公司" />
                            </div>
                            <div class="form-group">
                                <label for="assetPrice">当前价格</label>
                                <input id="assetPrice" v-model.number="newAsset.price" type="number" step="0.01"
                                    class="form-control" required placeholder="例如: 180.25" />
                            </div>
                            <button type="submit" class="submit-button" :disabled="creatingAsset">
                                <LoaderIcon v-if="creatingAsset" class="spinner" />
                                创建并添加
                            </button>
                        </form>
                    </div>
                </div>

                <div v-else class="search-results">
                    <div v-for="asset in searchResults" :key="asset.id" class="asset-item"
                        @click="addAssetToGroup(asset)">
                        <div class="asset-info">
                            <div class="asset-code">{{ asset.code }}</div>
                            <div class="asset-name">{{ asset.name }}</div>
                        </div>
                        <div class="asset-price">{{ formatPrice(asset.current_price) }}</div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { XIcon, SearchIcon, LoaderIcon, AlertCircleIcon, SearchXIcon } from 'lucide-vue-next';
import { useAssetStore, Asset } from '../../../../stores/assetStore';

const props = defineProps<{
    show: boolean;
    groupId: number;
}>();

const emit = defineEmits(['close', 'added']);

const assetStore = useAssetStore();
const loading = ref(false);
const error = ref<string | null>(null);
const searchQuery = ref('');
const searchResults = ref<Asset[]>([]);
const creatingAsset = ref(false);

// 新资产表单数据
const newAsset = ref({
    code: '',
    name: '',
    price: 0
});

// 搜索资产
const handleSearch = async () => {
    if (!searchQuery.value.trim()) {
        searchResults.value = [];
        return;
    }

    try {
        loading.value = true;
        error.value = null;

        // 在现有资产中搜索
        const query = searchQuery.value.toLowerCase();
        searchResults.value = assetStore.userAssets.filter(asset =>
            asset.code.toLowerCase().includes(query) ||
            asset.name.toLowerCase().includes(query)
        );

        // 如果需要，这里可以添加从API搜索资产的逻辑
        // 例如: const apiResults = await searchAssetsFromAPI(searchQuery.value);
        // searchResults.value = [...searchResults.value, ...apiResults];

    } catch (err) {
        error.value = err instanceof Error ? err.message : String(err);
        console.error('Search failed:', err);
    } finally {
        loading.value = false;
    }
};

// 添加资产到分组
const addAssetToGroup = async (asset: Asset) => {
    try {
        loading.value = true;
        error.value = null;

        // 检查资产是否已在该分组中
        const existingAsset = assetStore.userAssets.find(a =>
            a.code === asset.code && a.group_id === props.groupId
        );

        if (existingAsset) {
            error.value = '该资产已在分组中';
            return;
        }

        // 获取分组信息
        const group = assetStore.userGroups.find(g => g.id === props.groupId);
        if (!group) {
            throw new Error('分组不存在');
        }

        // 创建新资产
        const newAsset = await assetStore.createAsset(
            props.groupId,
            group.asset_type_id,
            asset.code,
            asset.name,
            asset.current_price
        );

        emit('added', newAsset);
        emit('close');
    } catch (err) {
        error.value = err instanceof Error ? err.message : String(err);
        console.error('Failed to add asset:', err);
    } finally {
        loading.value = false;
    }
};

// 创建新资产
const handleCreateAsset = async () => {
    try {
        creatingAsset.value = true;
        error.value = null;

        // 获取分组信息
        const group = assetStore.userGroups.find(g => g.id === props.groupId);
        if (!group) {
            throw new Error('分组不存在');
        }

        // 创建新资产
        const asset = await assetStore.createAsset(
            props.groupId,
            group.asset_type_id,
            newAsset.value.code,
            newAsset.value.name,
            newAsset.value.price
        );

        emit('added', asset);
        emit('close');
    } catch (err) {
        error.value = err instanceof Error ? err.message : String(err);
        console.error('Failed to create asset:', err);
    } finally {
        creatingAsset.value = false;
    }
};

// 格式化价格
const formatPrice = (price: number): string => {
    return price.toLocaleString('zh-CN', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
    });
};

// 初始化
onMounted(() => {
    // 重置表单
    newAsset.value = {
        code: searchQuery.value.toUpperCase(),
        name: '',
        price: 0
    };
});
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

.search-container {
    position: relative;
    margin-bottom: 16px;

    .search-input {
        width: 100%;
        padding: 10px 12px 10px 36px;
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

    .search-icon {
        position: absolute;
        left: 12px;
        top: 50%;
        transform: translateY(-50%);
        width: 16px;
        height: 16px;
        color: var(--textSecondary);
    }
}

.loading-container,
.error-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px 0;
    color: var(--textSecondary);
    gap: 12px;

    svg {
        width: 24px;
        height: 24px;
    }
}

.error-container {
    color: var(--errorColor);
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

.search-results {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-height: 400px;
    overflow-y: auto;
}

.asset-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px;
    border-radius: 4px;
    background-color: var(--cardBg);
    border: 1px solid var(--borderColor);
    cursor: pointer;
    transition: background-color 0.2s;

    &:hover {
        background-color: var(--hover-bg);
    }
}

.asset-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.asset-code {
    font-weight: 500;
    font-size: 14px;
}

.asset-name {
    font-size: 12px;
    color: var(--textSecondary);
}

.asset-price {
    font-weight: 500;
    font-size: 14px;
}

.empty-results {
    display: flex;
    flex-direction: column;
    gap: 24px;
}

.empty-message {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 24px 0;
    color: var(--textSecondary);
    gap: 12px;

    svg {
        width: 24px;
        height: 24px;
    }
}

.create-new {
    background-color: var(--cardBg);
    border: 1px solid var(--borderColor);
    border-radius: 8px;
    padding: 16px;

    p {
        margin: 0 0 16px 0;
        font-weight: 500;
    }
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

.submit-button {
    width: 100%;
    padding: 10px 16px;
    border-radius: 4px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
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
</style>