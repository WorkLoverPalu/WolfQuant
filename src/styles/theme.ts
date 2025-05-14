// 主题类型定义
export type ThemeType = "light" | "dark" | "system"

// 主题配置
export interface ThemeConfig {
  bgColor: string
  cardBg: string
  headerBg: string
  borderColor: string
  textColor: string
  textSecondary: string
  accentColor: string
  positiveColor: string
  negativeColor: string
  hoverBg: string
  activeBg: string
  inputBg: string
  buttonBg: string
  buttonHoverBg: string
  modalBg: string
  scrollbarThumb: string
  toolsBg: string
  resizerColor: string
  resizerHoverColor: string
  chartGridColor: string
}

// 主题配置映射
const themeConfigs: Record<Exclude<ThemeType, "system">, ThemeConfig> = {
  dark: {
    bgColor: "#121212",
    cardBg: "#1e1e1e",
    headerBg: "#1a1a1a",
    borderColor: "#333333",
    textColor: "#ffffff",
    textSecondary: "#a0a0a0",
    accentColor: "#2962ff",
    positiveColor: "#26a69a",
    negativeColor: "#ef5350",
    hoverBg: "rgba(255, 255, 255, 0.05)",
    activeBg: "rgba(255, 255, 255, 0.1)",
    inputBg: "#2c2c2c",
    buttonBg: "#2962ff",
    buttonHoverBg: "#1e53e4",
    modalBg: "#1e1e1e",
    scrollbarThumb: "#555555",
    toolsBg: "#0a0a0a",
    resizerColor: "#444444",
    resizerHoverColor: "#2962ff",
    chartGridColor: "rgba(255, 255, 255, 0.05)",
  },
  light: {
    bgColor: "#f5f5f5",
    cardBg: "#ffffff",
    headerBg: "#ffffff",
    borderColor: "#e0e0e0",
    textColor: "#333333",
    textSecondary: "#666666",
    accentColor: "#1a73e8",
    positiveColor: "#4caf50",
    negativeColor: "#f44336",
    hoverBg: "rgba(0, 0, 0, 0.03)",
    activeBg: "rgba(0, 0, 0, 0.05)",
    inputBg: "#f5f5f5",
    buttonBg: "#1a73e8",
    buttonHoverBg: "#1967d2",
    modalBg: "#ffffff",
    scrollbarThumb: "#cccccc",
    toolsBg: "#f0f0f0",
    resizerColor: "#dddddd",
    resizerHoverColor: "#1a73e8",
    chartGridColor: "rgba(0, 0, 0, 0.05)",
  },
}

// 获取系统主题偏好
const getSystemTheme = (): Exclude<ThemeType, "system"> => {
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light"
}

// 应用主题到文档
const applyThemeToDocument = (theme: ThemeConfig): void => {
  const root = document.documentElement

  Object.entries(theme).forEach(([key, value]) => {
    root.style.setProperty(`--${key}`, value)
  })
}

// 主题服务
export const themeService = {
  // 获取主题偏好
  getThemePreference(): ThemeType {
    return (localStorage.getItem("theme") as ThemeType) || "system"
  },

  // 保存主题偏好
  saveThemePreference(theme: ThemeType): void {
    localStorage.setItem("theme", theme)
  },

  // 应用主题
  applyTheme(theme: ThemeType): void {
    const actualTheme = theme === "system" ? getSystemTheme() : theme
    applyThemeToDocument(themeConfigs[actualTheme])

    // 更新body类名，用于CSS选择器
    if (actualTheme === "dark") {
      document.body.classList.add("dark-theme")
      document.body.classList.remove("light-theme")
    } else {
      document.body.classList.add("light-theme")
      document.body.classList.remove("dark-theme")
    }
  },

  // 监听系统主题变化
  watchSystemTheme(callback: () => void): () => void {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)")
    const listener = () => callback()

    mediaQuery.addEventListener("change", listener)
    return () => mediaQuery.removeEventListener("change", listener)
  },
}
