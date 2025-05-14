import { defineStore } from "pinia"
import { ref, watch } from "vue"
import { themeService, type ThemeType } from "../styles/theme"
import { useUserStore } from "./userStore"

export const useThemeStore = defineStore("theme", () => {
  // 当前主题
  const currentTheme = ref<ThemeType>(themeService.getThemePreference())

  // 监听主题变化
  watch(currentTheme, (newTheme) => {
    themeService.applyTheme(newTheme)
    themeService.saveThemePreference(newTheme)

    // 如果用户已登录，更新用户偏好设置
    const userStore = useUserStore()
    if (userStore.isAuthenticated) {
      userStore.updateUserPreferences({ theme: newTheme })
    }
  })

  // 设置主题
  function setTheme(theme: ThemeType) {
    currentTheme.value = theme
  }

  // 切换主题
  function toggleTheme() {
    const newTheme = currentTheme.value === "dark" ? "light" : "dark"
    setTheme(newTheme)
  }

  // 初始化主题
  function initTheme() {
    // 应用当前主题
    themeService.applyTheme(currentTheme.value)

    // 如果是系统主题，监听系统主题变化
    if (currentTheme.value === "system") {
      themeService.watchSystemTheme(() => {
        if (currentTheme.value === "system") {
          themeService.applyTheme("system")
        }
      })
    }
  }

  return {
    currentTheme,
    setTheme,
    toggleTheme,
    initTheme,
  }
})
