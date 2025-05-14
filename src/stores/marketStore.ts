import { defineStore } from "pinia"
import { ref, computed } from "vue"

export interface MarketData {
  name: string
  symbol: string
  price: string
  change: string
  percentChange: string
  high?: string
  low?: string
  open?: string
  prevClose?: string
  volume?: string
  turnover?: string
  updateTime?: string
}

export interface WatchlistGroup {
  id: string
  name: string
  category: string
  items: WatchlistItem[]
}

export interface WatchlistItem {
  symbol: string
  name: string
  price: string
  unit: string
  change: string
  changePercent: string
  volume: string
  turnover: string
}

export interface Position {
  cost: number
  amount: number
  investmentType?: "daily" | "weekly" | "biweekly" | "monthly"
  dayOfWeek?: number
  dayOfMonth?: number
  investmentAmount?: number
}

export const useMarketStore = defineStore("market", () => {
  // 当前市场数据
  const currentMarketData = ref<MarketData>({
    name: "上证指数",
    symbol: "sh000001",
    price: "3342.00",
    change: "-10.00",
    percentChange: "-0.30%",
    high: "3351.22",
    low: "3335.13",
    open: "3350.41",
    prevClose: "3352.00",
    volume: "4648.62亿",
    updateTime: "2025-05-09 15:30:39",
  })

  // 分组数据
  const groups = ref<WatchlistGroup[]>([
    {
      id: "indices",
      name: "INDICES",
      category: "stock",
      items: [
        {
          symbol: "SPX",
          name: "标准普尔500指数",
          price: "5,659.90",
          unit: "USD",
          change: "-4.05",
          changePercent: "-0.07%",
          volume: "2.39B",
          turnover: "2.91B",
        },
        {
          symbol: "NDQ",
          name: "US 100 Index",
          price: "20,061.45",
          unit: "USD",
          change: "-2.12",
          changePercent: "-0.01%",
          volume: "—",
          turnover: "—",
        },
        {
          symbol: "DJI",
          name: "道琼斯工业股票平均价格指数",
          price: "41,249.38",
          unit: "USD",
          change: "-119.07",
          changePercent: "-0.29%",
          volume: "—",
          turnover: "—",
        },
        {
          symbol: "VIX",
          name: "标普500波动率指数",
          price: "21.90",
          unit: "POINT",
          change: "-0.58",
          changePercent: "-2.58%",
          volume: "—",
          turnover: "—",
        },
        {
          symbol: "DXY",
          name: "美元指数",
          price: "100.424",
          unit: "USD",
          change: "-0.212",
          changePercent: "-0.21%",
          volume: "—",
          turnover: "—",
        },
      ],
    },
    {
      id: "crypto",
      name: "数字货币",
      category: "crypto",
      items: [
        {
          symbol: "BTCUSD",
          name: "比特币/美元",
          price: "67,890.50",
          unit: "USD",
          change: "+1,234.56",
          changePercent: "+1.85%",
          volume: "32.5B",
          turnover: "45.7B",
        },
        {
          symbol: "ETHUSD",
          name: "以太坊/美元",
          price: "3,456.78",
          unit: "USD",
          change: "+98.76",
          changePercent: "+2.94%",
          volume: "15.2B",
          turnover: "22.3B",
        },
      ],
    },
    {
      id: "gold",
      name: "黄金",
      category: "gold",
      items: [
        {
          symbol: "XAUUSD",
          name: "黄金/美元",
          price: "2,345.67",
          unit: "USD",
          change: "+12.34",
          changePercent: "+0.53%",
          volume: "1.2B",
          turnover: "3.4B",
        },
      ],
    },
    {
      id: "funds",
      name: "基金",
      category: "fund",
      items: [
        {
          symbol: "518880",
          name: "黄金基金",
          price: "2.456",
          unit: "CNY",
          change: "+0.023",
          changePercent: "+0.95%",
          volume: "123.4M",
          turnover: "345.6M",
        },
      ],
    },
  ])

  // 持仓数据
  const positions = ref<Record<string, Position>>({
    BTCUSD: { cost: 65000, amount: 10000 },
    ETHUSD: { cost: 3200, amount: 5000 },
    XAUUSD: { cost: 2300, amount: 8000 },
    "518880": { cost: 2.4, amount: 3000 },
  })

  // 当前选中的商品
  const selectedSymbol = ref<WatchlistItem | null>(null)

  // 分类数据
  const categories = [
    { id: "fund", name: "基金" },
    { id: "stock", name: "股票" },
    { id: "gold", name: "黄金" },
    { id: "crypto", name: "数字货币" },
  ]

  // 当前激活的分类
  const activeCategory = ref("fund")

  // 根据当前分类过滤分组
  const filteredGroups = computed(() => {
    if (activeCategory.value === "all") {
      return groups.value
    }
    return groups.value.filter((group) => group.category === activeCategory.value)
  })

  // 设置激活分类
  function setActiveCategory(categoryId: string) {
    activeCategory.value = categoryId
  }

  // 选择商品
  function selectSymbol(item: WatchlistItem) {
    selectedSymbol.value = item
  }

  // 添加分组
  function addGroup(name: string, category: string) {
    const id = `group-${Date.now()}`
    groups.value.push({
      id,
      name,
      category,
      items: [],
    })
    return id
  }

  // 编辑分组
  function editGroup(id: string, name: string, category: string) {
    const group = groups.value.find((g) => g.id === id)
    if (group) {
      group.name = name
      group.category = category
    }
  }

  // 删除分组
  function deleteGroup(id: string) {
    const index = groups.value.findIndex((g) => g.id === id)
    if (index !== -1) {
      groups.value.splice(index, 1)
    }
  }

  // 添加商品到分组
  function addSymbolToGroup(groupId: string, item: WatchlistItem) {
    const group = groups.value.find((g) => g.id === groupId)
    if (group) {
      // 检查是否已存在
      const exists = group.items.some((i) => i.symbol === item.symbol)
      if (!exists) {
        group.items.push(item)
      }
    }
  }

  // 从分组中移除商品
  function removeSymbolFromGroup(groupId: string, symbol: string) {
    const group = groups.value.find((g) => g.id === groupId)
    if (group) {
      const index = group.items.findIndex((i) => i.symbol === symbol)
      if (index !== -1) {
        group.items.splice(index, 1)
      }
    }
  }

  // 更新持仓
  function updatePosition(symbol: string, position: Position) {
    positions.value[symbol] = position
  }

  // 删除持仓
  function deletePosition(symbol: string) {
    delete positions.value[symbol]
  }

  return {
    currentMarketData,
    groups,
    positions,
    selectedSymbol,
    categories,
    activeCategory,
    filteredGroups,
    setActiveCategory,
    selectSymbol,
    addGroup,
    editGroup,
    deleteGroup,
    addSymbolToGroup,
    removeSymbolFromGroup,
    updatePosition,
    deletePosition,
  }
})
