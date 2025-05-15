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
      const tokenVerify = await invoke("auth_verify_session_command", {
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
  async function login(usernameOrEmail: string, password: string) {
    isLoading.value = true
    error.value = null

    try {
      // 调用 Tauri API 进行登录
      const response: any = await invoke('auth_login_command', {
        request: {
          username_or_email: usernameOrEmail,
          password: password
        }
      })

      // 保存用户数据和token
      user.value = response.user
      token.value = response.token
      localStorage.setItem("user", JSON.stringify(response.user))
      localStorage.setItem("auth_token", response.token)

      return response.user
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "登录失败"
      error.value = errorMessage
      throw new Error(errorMessage)
    } finally {
      isLoading.value = false
    }
  }

  // 发送验证码
  async function sendVerificationCode(email: string, purpose: 'register' | 'reset_password') {
    isLoading.value = true
    error.value = null

    try {
      await invoke('auth_send_verification_code_command', {
        request: {
          email: email,
          purpose: purpose
        }
      })
      return true
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "发送验证码失败"
      error.value = errorMessage
      throw new Error(errorMessage)
    } finally {
      isLoading.value = false
    }
  }

  // 注册
  async function register(username: string, email: string, password: string, verificationCode: string) {
    isLoading.value = true
    error.value = null

    try {
      const response: any = await invoke('auth_register_command', {
        request: {
          email: email,
          verification_code: verificationCode,
          username: username,
          password: password
        }
      })

      // 保存用户数据和token
      user.value = response.user
      token.value = response.token
      localStorage.setItem("user", JSON.stringify(response.user))
      localStorage.setItem("auth_token", response.token)

      return {
        user: response.user,
        message: response.message
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "注册失败"
      error.value = errorMessage
      throw new Error(errorMessage)
    } finally {
      isLoading.value = false
    }
  }

  // 登出
  async function logout() {
    if (!user.value || !token.value) return

    try {
      await invoke('auth_logout_command', {
        request: {
          user_id: user.value.id,
          token: token.value
        }
      })
      clearUserData()
      return true
    } catch (err) {
      console.error("Logout failed:", err)
      // 即使API调用失败，也清除本地数据
      clearUserData()
      throw err
    }
  }

  // 清除用户数据
  function clearUserData() {
    user.value = null
    token.value = null
    localStorage.removeItem("user")
    localStorage.removeItem("auth_token")
  }

  // 重置密码
  async function resetPassword(email: string, code: string, newPassword: string) {

    const response: any = await invoke('auth_forgot_password_command', {
      request: {
        email: email,
        verification_code: code,
        new_password: newPassword,
      },
    })
    return response

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
    sendVerificationCode,
    register,
    logout,
    resetPassword,
    updateUserPreferences,
    clearUserData
  }
})