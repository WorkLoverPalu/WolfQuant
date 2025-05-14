<template>
    <div class="market-footer" :class="{ 'light-theme': theme === 'light' }">
        <div class="left">
            <!-- 左侧账户信息 -->
            <div class="footer-section account-section" v-if="showAccount && currentUser">
                <div class="account-item">
                    <span>股票：</span>
                    <DollarSignIcon class="icon" />
                    <span class="value">{{ formatNumber(accountInfo.balance) }}</span>
                    <span class="separator">|</span>
                    <span class="value">{{ formatNumber(accountInfo.available) }}</span>
                </div>
            </div>
            <div class="footer-section account-section" v-if="showAccount && currentUser">
                <div class="account-item">
                    <span>基金：</span>
                    <DollarSignIcon class="icon" />
                    <span class="value">{{ formatNumber(accountInfo.balance) }}</span>
                    <span class="separator">|</span>
                    <span class="value">{{ formatNumber(accountInfo.available) }}</span>
                </div>
            </div>
            <div class="footer-section account-section" v-if="showAccount && currentUser">
                <div class="account-item">
                    <span>数字货币：</span>
                    <DollarSignIcon class="icon" />
                    <span class="value">{{ formatNumber(accountInfo.balance) }}</span>
                    <span class="separator">|</span>
                    <span class="value">{{ formatNumber(accountInfo.available) }}</span>
                </div>
            </div>

            <!-- 中间行情信息 -->
            <div class="footer-section market-section" @click="toggleMarketDetail">
                <div class="market-info">
                    <span class="market-name">「{{ currentMarket.name }}」</span>
                    <span class="market-code">{{ currentMarket.symbol }}</span>
                    <span class="market-price">{{ currentMarket.price }}</span>
                    <span class="market-change" :class="{
                        'positive': parseFloat(currentMarket.change) > 0,
                        'negative': parseFloat(currentMarket.change) < 0
                    }">
                        涨跌: {{ currentMarket.change }}
                    </span>
                    <span class="market-percent" :class="{
                        'positive': parseFloat(currentMarket.percentChange) > 0,
                        'negative': parseFloat(currentMarket.percentChange) < 0
                    }">
                        百分: {{ currentMarket.percentChange }}
                    </span>
                </div>

                <!-- 行情详情弹出框 -->
                <transition name="slide-up">
                    <div v-if="showMarketDetail" class="market-detail">
                        <div class="detail-row">
                            <div class="detail-item">
                                <span class="detail-label">最高:</span>
                                <span class="detail-value">{{ currentMarket.high }}</span>
                            </div>
                            <div class="detail-item">
                                <span class="detail-label">最低:</span>
                                <span class="detail-value">{{ currentMarket.low }}</span>
                            </div>
                        </div>
                        <div class="detail-row">
                            <div class="detail-item">
                                <span class="detail-label">今开:</span>
                                <span class="detail-value">{{ currentMarket.open }}</span>
                            </div>
                            <div class="detail-item">
                                <span class="detail-label">昨收:</span>
                                <span class="detail-value">{{ currentMarket.prevClose }}</span>
                            </div>
                        </div>
                        <div class="detail-row">
                            <div class="detail-item">
                                <span class="detail-label">成交额:</span>
                                <span class="detail-value">{{ currentMarket.volume }}</span>
                            </div>
                        </div>
                        <div class="detail-row">
                            <div class="detail-item">
                                <span class="detail-label">更新时间:</span>
                                <span class="detail-value">{{ currentMarket.updateTime }}</span>
                            </div>
                        </div>
                    </div>
                </transition>
            </div>
        </div>


        <!-- 右侧导航按钮 -->
        <div class="footer-section nav-section" v-if="showNav">
            <div v-for="(item, index) in navItems" :key="index" class="nav-item"
                :class="{ active: activeNavItem === index }" @click="handleNavClick(index, item)">
                <component :is="item.icon" class="nav-icon" />
                <span v-if="item.label" class="nav-label">{{ item.label }}</span>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, inject } from 'vue';
import {
    DollarSignIcon,
    HomeIcon,
    BarChart2Icon,
    SearchIcon,
    UserIcon,
    BellIcon
} from 'lucide-vue-next';

// 定义组件属性
interface Props {
    // 是否显示账户信息
    showAccount?: boolean;
    // 是否显示导航按钮
    showNav?: boolean;
    // 自定义市场数据
    marketData?: MarketData;
    // 当前用户
    currentUser?: any;
}

const props = withDefaults(defineProps<Props>(), {
    showAccount: true,
    showNav: true,
    marketData: () => ({
        name: '上证指数',
        symbol: 'sh000001',
        price: '3342.00',
        change: '-10.00',
        percentChange: '-0.30%',
        high: '3351.22',
        low: '3335.13',
        open: '3350.41',
        prevClose: '3352.00',
        volume: '4648.62亿',
        updateTime: '2025-05-09 15:30:39'
    }),
    currentUser: null
});

// 定义事件
const emit = defineEmits<{
    // 导航项点击事件
    (e: 'nav-click', index: number, item: NavItem): void;
    // 行情点击事件
    (e: 'market-click', market: MarketData): void;
    // 打开新标签页事件
    (e: 'open-tab', tabData: any): void;
}>();

// 市场数据类型
interface MarketData {
    name: string;
    symbol: string;
    price: string;
    change: string;
    percentChange: string;
    high: string;
    low: string;
    open: string;
    prevClose: string;
    volume: string;
    updateTime: string;
}

// 导航项类型
interface NavItem {
    icon: any;
    label?: string;
    route?: string;
    action?: () => void;
}

// 从ThemeProvider获取主题
const theme = inject('theme', ref('dark'));

// 账户信息
const accountInfo = ref({
    balance: 10000,
    available: 8500
});

// 当前市场数据
const currentMarket = ref<MarketData>(props.marketData);

// 行情详情显示状态
const showMarketDetail = ref(false);

// 导航项
const navItems: NavItem[] = [
    {
        icon: HomeIcon,
        label: '首页',
        action: () => emit('open-tab', {
            id: 'home',
            title: '首页',
            component: 'EmptyTab',
            closable: true
        })
    },
    {
        icon: BarChart2Icon,
        label: '行情',
        action: () => emit('open-tab', {
            id: 'market',
            title: '行情',
            component: 'MarketWatchlist',
            closable: true
        })
    },
    {
        icon: SearchIcon,
        label: '搜索',
        action: () => emit('open-tab', {
            id: 'search',
            title: '搜索',
            component: 'SearchTab',
            closable: true
        })
    },
    {
        icon: BellIcon,
        label: '通知',
        action: () => emit('open-tab', {
            id: 'notifications',
            title: '通知',
            component: 'NotificationsTab',
            closable: true
        })
    },
    {
        icon: UserIcon,
        label: '我的',
        action: () => emit('open-tab', {
            id: 'profile',
            title: '个人中心',
            component: 'UserProfile',
            props: { userData: props.currentUser },
            closable: true
        })
    }
];

// 当前激活的导航项
const activeNavItem = ref(0);

// 格式化数字
const formatNumber = (num: number): string => {
    return num.toLocaleString('zh-CN');
};

// 切换行情详情显示
const toggleMarketDetail = () => {
    showMarketDetail.value = !showMarketDetail.value;
    emit('market-click', currentMarket.value);
};

// 处理导航点击
const handleNavClick = (index: number, item: NavItem) => {
    activeNavItem.value = index;
    emit('nav-click', index, item);

    // 如果有自定义动作，执行它
    if (item.action) {
        item.action();
    }
};

// 更新市场数据
const updateMarketData = (data: Partial<MarketData>) => {
    currentMarket.value = { ...currentMarket.value, ...data };
};

// 组件挂载时
onMounted(() => {
    // 模拟行情数据更新
    const intervalId = setInterval(() => {
        const randomChange = (Math.random() * 2 - 1).toFixed(2);
        const currentPrice = parseFloat(currentMarket.value.price.replace(',', ''));
        const newPrice = (currentPrice + parseFloat(randomChange)).toFixed(2);
        const percentChange = ((parseFloat(randomChange) / currentPrice) * 100).toFixed(2);

        updateMarketData({
            price: newPrice,
            change: randomChange.startsWith('-') ? randomChange : `+${randomChange}`,
            percentChange: `${randomChange.startsWith('-') ? '' : '+'}${percentChange}%`,
            updateTime: new Date().toLocaleString('zh-CN')
        });
    }, 5000);

    // 清理定时器
    onUnmounted(() => {
        clearInterval(intervalId);
    });
});

// 暴露方法给父组件
defineExpose({
    updateMarketData,
    toggleMarketDetail
});
</script>

<style lang="scss" scoped>
.market-footer {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    height: 48px;
    background-color: var(--header-bg, #1a1a1a);
    color: var(--tab-active-text, #ffffff);
    display: flex;
    align-items: center;
    border-top: 1px solid var(--border-color, #333333);
    z-index: 1000;
    font-size: 14px;
    justify-content: space-between;

    &.light-theme {
        --header-bg: var(--header-bg, #ffffff);
        --tab-active-text: var(--tab-active-text, #000000);
        --border-color: var(--border-color, #dddddd);
        --tab-text: var(--tab-text, #666666);
        --button-primary: var(--button-primary, #2563eb);
        --positive-color: #4caf50;
        --negative-color: #f44336;
    }
}
.left{
    display: flex;
    align-items: center;
}

.footer-section {
    height: 100%;
    display: flex;
    align-items: center;
}

.account-section {
    width: auto;
    padding: 0 12px;
    overflow: hidden;
}

.account-item {
    display: flex;
    align-items: center;
    font-size: 12px;

    .icon {
        width: 16px;
        height: 16px;
        margin-right: 4px;
        color: var(--button-primary, #2563eb);
    }

    .value {
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .separator {
        margin: 0 4px;
        color: var(--tab-text, #a0a0a0);
    }
}

.market-section {
    width: auto;
    justify-content: center;
    position: relative;
    cursor: pointer;
}

.market-info {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-wrap: wrap;
    padding: 0 8px;
    gap: 6px;
    max-width: 100%;
    overflow: hidden;
}

.market-name,
.market-code {
    font-weight: 500;
}

.market-price {
    font-weight: 600;
}

.market-change,
.market-percent {
    &.positive {
        color: var(--positive-color, #26a69a);
    }

    &.negative {
        color: var(--negative-color, #ef5350);
    }
}

.market-detail {
    position: absolute;
    bottom: 48px;
    left: 50%;
    transform: translateX(-50%);
    width: 300px;
    background-color: var(--header-bg, #1a1a1a);
    border: 1px solid var(--border-color, #333333);
    border-bottom: none;
    border-radius: 8px 8px 0 0;
    padding: 12px;
    box-shadow: 0 -4px 12px rgba(0, 0, 0, 0.1);
}

.detail-row {
    display: flex;
    justify-content: space-between;
    margin-bottom: 8px;

    &:last-child {
        margin-bottom: 0;
    }
}

.detail-item {
    display: flex;
    align-items: center;
}

.detail-label {
    color: var(--tab-text, #a0a0a0);
    margin-right: 4px;
    font-size: 12px;
}

.detail-value {
    font-size: 13px;
    font-weight: 500;
}

.nav-section {
    width: 25%;
    justify-content: space-around;
    float: right;
}

.nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 0 8px;
    cursor: pointer;

    &.active {
        color: var(--button-primary, #2563eb);
    }

    &:hover {
        background-color: rgba(128, 128, 128, 0.1);
    }
}

.nav-icon {
    width: 18px;
    height: 18px;
}

.nav-label {
    font-size: 10px;
    margin-top: 2px;
}

/* 动画 */
.slide-up-enter-active,
.slide-up-leave-active {
    transition: transform 0.3s ease, opacity 0.3s ease;
}

.slide-up-enter-from,
.slide-up-leave-to {
    transform: translateX(-50%) translateY(20px);
    opacity: 0;
}

/* 主题变量 */
:root {
    --positive-color: #26a69a;
    --negative-color: #ef5350;
}
</style>