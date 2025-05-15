import { defineStore } from "pinia"
import { ref, computed } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { useUserStore } from "./userStore"

// Existing interfaces
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

// New interfaces for Tauri backend
export interface AssetType {
  id: number
  name: string
  code: string
  description?: string
}

export interface UserGroup {
  id: number
  user_id: string
  name: string
  asset_type_id: number
  description?: string
  created_at: string
  updated_at: string
}

export interface Asset {
  id: number
  user_id: string
  group_id: number
  asset_type_id: number
  code: string
  name: string
  current_price: number
  created_at: string
  updated_at: string
}

// Request types for Tauri backend
export interface CreateGroupRequest {
  user_id: string
  name: string
  asset_type_id: number
  description?: string
}

export interface UpdateGroupRequest {
  id: number
  user_id: string
  name: string
  description?: string
}

export interface DeleteGroupRequest {
  id: number
  user_id: string
}

export interface CreateAssetRequest {
  user_id: string
  group_id: number
  asset_type_id: number
  code: string
  name: string
  current_price: number
}

export interface UpdateAssetRequest {
  id: number
  user_id: string
  group_id: number
  name: string
  current_price: number
}

export interface DeleteAssetRequest {
  id: number
  user_id: string
}

export interface GetUserAssetsRequest {
  user_id: string
  asset_type_id?: number
  group_id?: number
}

export interface MessageResponse {
  message: string
}

export const useAssetStore = defineStore("market", () => {
  const userStore = useUserStore()

  // ===== EXISTING MARKET STORE STATE =====

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

  // 当前激活的分类
  const activeCategory = ref("fund")

  // 根据当前分类过滤分组
  const filteredGroups = computed(() => {
    if (activeCategory.value === "all") {
      return groups.value
    }
    return groups.value.filter((group) => group.category === activeCategory.value)
  })

  // ===== NEW ASSET MANAGEMENT STATE =====

  const assetTypes = ref<AssetType[]>([])
  const userGroups = ref<UserGroup[]>([])
  const userAssets = ref<Asset[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Computed properties for asset management
  const groupsByType = computed(() => {
    const result: Record<number, UserGroup[]> = {}

    userGroups.value.forEach(group => {
      if (!result[group.asset_type_id]) {
        result[group.asset_type_id] = []
      }
      result[group.asset_type_id].push(group)
    })

    return result
  })

  const assetsByGroup = computed(() => {
    const result: Record<number, Asset[]> = {}

    userAssets.value.forEach(asset => {
      if (!result[asset.group_id]) {
        result[asset.group_id] = []
      }
      result[asset.group_id].push(asset)
    })

    return result
  })

  // ===== EXISTING MARKET STORE METHODS =====

  // 设置激活分类
  function setActiveCategory(categoryId: string) {
    activeCategory.value = categoryId
  }

  // 选择商品
  function selectSymbol(item: WatchlistItem) {
    selectedSymbol.value = item
  }

  // =====资产管理方法 =====

  // 获取所有资产类型
  async function fetchAssetTypes() {
    loading.value = true
    error.value = null

    try {
      const types = await invoke<AssetType[]>("asset_get_asset_types_command")
      assetTypes.value = types
      return types
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error("Failed to fetch asset types:", err)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 获取用户组
  async function fetchUserGroups(assetTypeId?: number) {
    if (!userStore.user?.id) {
      throw new Error("User not authenticated")
    }

    loading.value = true
    error.value = null

    try {
      const groups = await invoke<UserGroup[]>("asset_get_user_groups_command", {
        userId: userStore.user.id,
        assetTypeId: assetTypeId
      })

      // If fetching for a specific type, merge with existing groups
      if (assetTypeId) {
        const filteredGroups = userGroups.value.filter(g => g.asset_type_id !== assetTypeId)
        userGroups.value = [...filteredGroups, ...groups]
      } else {
        userGroups.value = groups
      }

      return groups
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error("Failed to fetch user groups:", err)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 创建一个新的用户组
  async function createUserGroup(name: string, assetTypeId: number, description?: string) {
    if (!userStore.user?.id) {
      throw new Error("User not authenticated")
    }

    loading.value = true
    error.value = null

    const request: CreateGroupRequest = {
      user_id: userStore.user.id,
      name,
      asset_type_id: assetTypeId,
      description
    }

    try {
      const newGroup = await invoke<UserGroup>("asset_create_group_command", { request })
      userGroups.value.push(newGroup)
      return newGroup
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error("Failed to create group:", err)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 更新用户分组
  async function updateUserGroup(groupId: number, name: string, description?: string) {
    if (!userStore.user?.id) {
      throw new Error("User not authenticated")
    }

    loading.value = true
    error.value = null

    const request: UpdateGroupRequest = {
      id: groupId,
      user_id: userStore.user.id,
      name,
      description
    }

    try {
      const updatedGroup = await invoke<UserGroup>("asset_update_group_command", { request })

      // Update the group in the local state
      const index = userGroups.value.findIndex(g => g.id === groupId)
      if (index !== -1) {
        userGroups.value[index] = updatedGroup
      }

      return updatedGroup
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error("Failed to update group:", err)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 删除用户组
  async function deleteUserGroup(groupId: number) {
    if (!userStore.user?.id) {
      throw new Error("User not authenticated")
    }

    loading.value = true
    error.value = null

    const request: DeleteGroupRequest = {
      id: groupId,
      user_id: userStore.user.id
    }

    try {
      const response = await invoke<MessageResponse>("asset_delete_group_command", { request })

      // Remove the group from local state
      userGroups.value = userGroups.value.filter(g => g.id !== groupId)

      return response
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error("Failed to delete group:", err)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 获取用户资产
  async function fetchUserAssets(assetTypeId?: number, groupId?: number) {
    if (!userStore.user?.id) {
      throw new Error("User not authenticated")
    }

    loading.value = true
    error.value = null

    const request: GetUserAssetsRequest = {
      user_id: userStore.user.id,
      asset_type_id: assetTypeId,
      group_id: groupId
    }

    try {
      const assets = await invoke<Asset[]>("asset_get_user_assets_command", { request })

      // If fetching for a specific group or type, merge with existing assets
      if (assetTypeId || groupId) {
        let filteredAssets = userAssets.value

        if (assetTypeId) {
          filteredAssets = filteredAssets.filter(a => a.asset_type_id !== assetTypeId)
        }

        if (groupId) {
          filteredAssets = filteredAssets.filter(a => a.group_id !== groupId)
        }

        userAssets.value = [...filteredAssets, ...assets]
      } else {
        userAssets.value = assets
      }

      return assets
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error("Failed to fetch user assets:", err)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 创建一个新资产
  async function createAsset(
    groupId: number,
    assetTypeId: number,
    code: string,
    name: string,
    currentPrice: number
  ) {
    if (!userStore.user?.id) {
      throw new Error("User not authenticated")
    }

    loading.value = true
    error.value = null

    const request: CreateAssetRequest = {
      user_id: userStore.user.id,
      group_id: groupId,
      asset_type_id: assetTypeId,
      code,
      name,
      current_price: currentPrice
    }

    try {
      const newAsset = await invoke<Asset>("asset_create_asset_command", { request })
      userAssets.value.push(newAsset)
      return newAsset
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error("Failed to create asset:", err)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 更新资产
  async function updateAsset(
    assetId: number,
    groupId: number,
    name: string,
    currentPrice: number
  ) {
    if (!userStore.user?.id) {
      throw new Error("User not authenticated")
    }

    loading.value = true
    error.value = null

    const request: UpdateAssetRequest = {
      id: assetId,
      user_id: userStore.user.id,
      group_id: groupId,
      name,
      current_price: currentPrice
    }

    try {
      const updatedAsset = await invoke<Asset>("asset_update_asset_command", { request })

      // Update the asset in the local state
      const index = userAssets.value.findIndex(a => a.id === assetId)
      if (index !== -1) {
        userAssets.value[index] = updatedAsset
      }

      return updatedAsset
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error("Failed to update asset:", err)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 删除资产
  async function deleteAsset(assetId: number) {
    if (!userStore.user?.id) {
      throw new Error("User not authenticated")
    }

    loading.value = true
    error.value = null

    const request: DeleteAssetRequest = {
      id: assetId,
      user_id: userStore.user.id
    }

    try {
      const response = await invoke<MessageResponse>("asset_delete_asset_command", { request })

      // Remove the asset from local state
      userAssets.value = userAssets.value.filter(a => a.id !== assetId)

      return response
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error("Failed to delete asset:", err)
      throw err
    } finally {
      loading.value = false
    }
  }

  // 初始化资产数据
  async function initAssetData() {
    console.log("***初始化资产数据*****");

    if (userStore.isAuthenticated) {
      try {
        await fetchAssetTypes()
        await fetchUserGroups()
        await fetchUserAssets()
      } catch (err) {
        console.error("Failed to initialize asset data:", err)
      }
    }
  }

  return {
    // ===== EXISTING MARKET STORE STATE =====
    currentMarketData,
    groups,
    positions,
    selectedSymbol,
    activeCategory,
    filteredGroups,

    // ===== EXISTING MARKET STORE METHODS =====
    setActiveCategory,
    selectSymbol,

    // ===== NEW ASSET MANAGEMENT STATE =====
    assetTypes,
    userGroups,
    userAssets,
    loading,
    error,
    groupsByType,
    assetsByGroup,

    // ===== NEW ASSET MANAGEMENT METHODS =====
    fetchAssetTypes,
    fetchUserGroups,
    createUserGroup,
    updateUserGroup,
    deleteUserGroup,
    fetchUserAssets,
    createAsset,
    updateAsset,
    deleteAsset,
    initAssetData
  }
})