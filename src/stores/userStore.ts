import { defineStore } from "pinia"
import { ref, computed } from "vue"
import { invoke } from "@tauri-apps/api/core"

export interface User {
  id: string
  username: string
  email: string
  avatar?: string
  preferences?: {
    theme?: "light" | "dark" | "system"
    language?: string
    notifications?: boolean
  }
}

export const useUserStore = defineStore("user", () => {
  // 状态
  const user = ref<User | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const token = ref<string | null>(null)

  // 计算属性
  const isAuthenticated = computed(() => !!user.value)
  const username = computed(() => user.value?.username || "")
  const userInitial = computed(() => user.value?.username.charAt(0).toUpperCase() || "")

  // 初始化用户
  async function initUser() {
    const savedToken = localStorage.getItem("auth_token")
    if (!savedToken) return

    token.value = savedToken

    try {
      const tokenVerify = await invoke("auth_verify_session", {
        request: { token: savedToken },
      })

      if (tokenVerify) {
        const savedUser = localStorage.getItem("user")
        if (savedUser) {
          user.value = JSON.parse(savedUser)
        }
      } else {
        clearUserData()
      }
    } catch (err) {
      clearUserData()
      console.error("Failed to verify token:", err)
    }
  }

  // 登录
  async function login(email: string, password: string) {
    isLoading.value = true
    error.value = null

    try {
      // 这里应该是实际的登录逻���，调用后端API
      // 模拟登录成功
      const userData: User = {
        id: "1",
        username: email.split("@")[0],
        email,
        avatar: email.charAt(0).toUpperCase(),
      }

      // 模拟获取token
      const authToken = "mock-token-" + Date.now()

      // 保存用户数据和token
      user.value = userData
      token.value = authToken
      localStorage.setItem("user", JSON.stringify(userData))
      localStorage.setItem("auth_token", authToken)

      return userData
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "登录失败"
      error.value = errorMessage
      throw new Error(errorMessage)
    } finally {
      isLoading.value = false
    }
  }

  // 注册
  async function register(username: string, email: string, password: string) {
    isLoading.value = true
    error.value = null

    try {
      // 这里应该是实际的注册逻辑，调用后端API
      // 模拟注册成功
      const userData: User = {
        id: "1",
        username,
        email,
        avatar: username.charAt(0).toUpperCase(),
      }

      // 模拟获取token
      const authToken = "mock-token-" + Date.now()

      // 保存用户数据和token
      user.value = userData
      token.value = authToken
      localStorage.setItem("user", JSON.stringify(userData))
      localStorage.setItem("auth_token", authToken)

      return userData
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "注册失败"
      error.value = errorMessage
      throw new Error(errorMessage)
    } finally {
      isLoading.value = false
    }
  }

  // 登出
  function logout() {
    clearUserData()
  }

  // 清除用户数据
  function clearUserData() {
    user.value = null
    token.value = null
    localStorage.removeItem("user")
    localStorage.removeItem("auth_token")
  }

  // 更新用户偏好设置
  function updateUserPreferences(preferences: Partial<User["preferences"]>) {
    if (!user.value) return

    user.value = {
      ...user.value,
      preferences: {
        ...user.value.preferences,
        ...preferences,
      },
    }

    localStorage.setItem("user", JSON.stringify(user.value))
  }

  // 重置密码
  async function resetPassword(email: string) {
    isLoading.value = true
    error.value = null

    try {
      // 这里应该是实际的重置密码逻辑，调用后端API
      // 模拟重置密码成功
      console.log(`重置密码链接已发送至: ${email}`)
      return true
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "重置密码失败"
      error.value = errorMessage
      throw new Error(errorMessage)
    } finally {
      isLoading.value = false
    }
  }

  return {
    user,
    isLoading,
    error,
    token,
    isAuthenticated,
    username,
    userInitial,
    initUser,
    login,
    register,
    logout,
    updateUserPreferences,
    resetPassword,
  }
})
