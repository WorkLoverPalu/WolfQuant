import { defineStore } from "pinia"
import { ref, computed, markRaw } from "vue"
import MarketWatchlist from "../views/marketWatch/MarketWatchlist.vue"
import EmptyTab from "../components/EmptyTab.vue"
import UserProfile from "../views/userProfile/UserProfile.vue"

export interface Tab {
  id: string
  title: string
  component: any
  props?: Record<string, any>
  closable: boolean
}

export const useTabStore = defineStore("tab", () => {
  // 标签页列表
  const tabs = ref<Tab[]>([
    {
      id: "market-watchlist",
      title: "MarketWatchlist",
      component: markRaw(MarketWatchlist),
      props: {},
      closable: true,
    },
    {
      id: "2",
      title: "BIOUSDT",
      component: "WolfQuant",
      props: { symbol: "BIOUSDT", price: "0.08221", change: "+16.1%" },
      closable: true,
    },
    {
      id: "3",
      title: "New Tab",
      component: markRaw(EmptyTab),
      closable: true,
    },
  ])

  // 当前激活的标签页索引
  const activeTabIndex = ref(0)

  // 计算属性：当前激活的标签页组件
  const activeTabComponent = computed(() => {
    return tabs.value[activeTabIndex.value]?.component || markRaw(EmptyTab)
  })

  // 计算属性：当前激活的标签页属性
  const activeTabProps = computed(() => {
    return tabs.value[activeTabIndex.value]?.props || {}
  })

  // 切换标签页
  function switchTab(index: number) {
    activeTabIndex.value = index
  }

  // 关闭标签页
  function closeTab(index: number) {
    if (tabs.value[index].closable) {
      tabs.value = tabs.value.filter((_, i) => i !== index)

      // 如果关闭的是当前活动标签页，切换到第一个标签页
      if (activeTabIndex.value === index) {
        activeTabIndex.value = 0
      } else if (activeTabIndex.value > index) {
        // 如果关闭的标签页在当前活动标签页之前，调整索引
        activeTabIndex.value--
      }
    }
  }

  // 添加新标签页
  function addNewTab(
    title = "New Tab",
    component: any = markRaw(EmptyTab),
    props: Record<string, any> = {},
    closable = true,
  ) {
    const newTabId = `tab-${Date.now()}`
    tabs.value.push({
      id: newTabId,
      title,
      component: typeof component === "string" ? component : markRaw(component),
      props,
      closable,
    })
    activeTabIndex.value = tabs.value.length - 1
    return newTabId
  }

  // 打开用户个人中心标签
  function openUserProfileTab(userData: any) {
    // 检查是否已经存在用户个人中心标签
    const existingTabIndex = tabs.value.findIndex((tab) => tab.title === "个人中心")

    if (existingTabIndex !== -1) {
      // 如果已存在，切换到该标签
      activeTabIndex.value = existingTabIndex
      // 更新用户数据
      tabs.value[existingTabIndex].props = { userData }
    } else {
      // 如果不存在，创建新标签
      addNewTab("个人中心", markRaw(UserProfile), { userData }, true)
    }
  }

  // 打开或切换到指定标签页
  function openTab(tabData: Partial<Tab> & { id: string }) {
    // 检查是否已经存在相同ID的标签
    const existingTabIndex = tabs.value.findIndex((tab) => tab.id === tabData.id)

    if (existingTabIndex !== -1) {
      // 如果已存在，切换到该标签
      activeTabIndex.value = existingTabIndex
      // 可选：更新标签属性
      if (tabData.props) {
        tabs.value[existingTabIndex].props = tabData.props
      }
    } else {
      // 如果不存在，创建新标签
      addNewTab(
        tabData.title || "New Tab",
        typeof tabData.component === "string" ? tabData.component : markRaw(tabData.component),
        tabData.props || {},
        tabData.closable !== undefined ? tabData.closable : true,
      )
    }
  }

  return {
    tabs,
    activeTabIndex,
    activeTabComponent,
    activeTabProps,
    switchTab,
    closeTab,
    addNewTab,
    openUserProfileTab,
    openTab,
  }
})
