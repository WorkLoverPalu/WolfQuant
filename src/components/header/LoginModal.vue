<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-container" @click.stop>
      <div class="modal-header">
        <h2>登录</h2>
        <button class="close-button" @click="$emit('close')">×</button>
      </div>

      <div class="modal-body">
        <div v-if="error" class="error-message">{{ error }}</div>

        <form @submit.prevent="handleSubmit">
          <div class="form-group">
            <label for="username">用户名或邮箱</label>
            <input type="text" id="username" v-model="username" placeholder="请输入用户名或邮箱" required
              :disabled="isLoading" />
          </div>

          <div class="form-group">
            <label for="password">密码</label>
            <input type="password" id="password" v-model="password" placeholder="请输入密码" required
              :disabled="isLoading" />
          </div>

          <div class="form-actions">
            <button type="submit" class="primary-button" :disabled="isLoading">
              {{ isLoading ? '登录中...' : '登录' }}
            </button>
          </div>
        </form>
      </div>

      <div class="modal-footer">
        <button class="text-button" @click="$emit('forgot-password')">
          忘记密码？
        </button>
        <button class="text-button" @click="$emit('register')">
          注册账号
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';


const username = ref('');
const password = ref('');
const isLoading = ref(false);
const error = ref('');

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'login-success', user: any, token: string): void;
  (e: 'forgot-password'): void;
  (e: 'register'): void;
}>();

const handleSubmit = async () => {
  if (!username.value || !password.value) return;

  isLoading.value = true;
  error.value = '';

  try {
    const response: any = await invoke('auth_login_command', {
      request: {
        username_or_email: username.value,
        password: password.value
      }
    });

    console.log("response", response)

    // 存储用户信息和令牌
    localStorage.setItem('auth_token', response.token);
    localStorage.setItem('user', JSON.stringify(response.user));

    // 通知父组件登录成功
    const user = {
      id: response.user.id,
      username: response.user.username,
      email: response.user.email,
      avatar: response.user.username.charAt(0).toUpperCase(),
    };
    emit('login-success', user, response.token);

    // 清空表单
    username.value = '';
    password.value = '';
  } catch (err: any) {
    error.value = err.error || '登录失败，请检查用户名和密码';
  } finally {
    isLoading.value = false;
  }
};
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
  background-color: var(--modal-bg);
  border-radius: 8px;
  width: 100%;
  max-width: 400px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid var(--border-color);

  h2 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
  }

  .close-button {
    background: transparent;
    border: none;
    color: var(--tab-text);
    font-size: 24px;
    cursor: pointer;

    &:hover {
      color: var(--tab-active-text);
    }
  }
}

.modal-body {
  padding: 16px;

  .error-message {
    margin-bottom: 16px;
    padding: 10px;
    background-color: rgba(220, 38, 38, 0.1);
    color: #dc2626;
    border-radius: 4px;
    font-size: 14px;
  }

  .form-group {
    margin-bottom: 16px;

    label {
      display: block;
      margin-bottom: 8px;
      font-size: 14px;
      color: var(--tab-text);
    }

    input {
      width: 100%;
      padding: 10px 12px;
      border-radius: 4px;
      border: 1px solid var(--border-color);
      background-color: var(--input-bg);
      color: var(--tab-active-text);
      font-size: 14px;

      &:focus {
        outline: none;
        border-color: var(--button-primary);
      }

      &::placeholder {
        color: var(--tab-text);
      }

      &:disabled {
        opacity: 0.7;
        cursor: not-allowed;
      }
    }
  }

  .form-actions {
    margin-top: 24px;

    .primary-button {
      width: 100%;
      padding: 10px;
      background-color: var(--button-primary);
      color: white;
      border: none;
      border-radius: 4px;
      font-size: 14px;
      font-weight: 500;
      cursor: pointer;

      &:hover:not(:disabled) {
        background-color: var(--button-primary-hover);
      }

      &:disabled {
        opacity: 0.7;
        cursor: not-allowed;
      }
    }
  }
}

.modal-footer {
  display: flex;
  justify-content: space-between;
  padding: 16px;
  border-top: 1px solid var(--border-color);

  .text-button {
    background: transparent;
    border: none;
    color: var(--button-primary);
    font-size: 14px;
    cursor: pointer;

    &:hover {
      text-decoration: underline;
    }
  }
}
</style>