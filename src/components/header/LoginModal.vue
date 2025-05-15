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
import { useUserStore } from '../../stores/userStore';

const username = ref('');
const password = ref('');
const isLoading = ref(false);
const error = ref('');

const userStore = useUserStore();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'login-success', user: any, token: any): void;
  (e: 'forgot-password'): void;
  (e: 'register'): void;
}>();

const handleSubmit = async () => {
  if (!username.value || !password.value) return;

  isLoading.value = true;
  error.value = '';

  try {
    // 使用 store 的 login 方法
    const user = await userStore.login(username.value, password.value);

    // 通知父组件登录成功
    emit('login-success', user, userStore.token);

    // 清空表单
    username.value = '';
    password.value = '';
  } catch (err: any) {
    error.value = err.message || '登录失败，请检查用户名和密码';
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
  z-index: var(--z-index-modal);
}

.modal-container {
  background-color: var(--modalBg);
  border-radius: var(--radius-md);
  width: 100%;
  max-width: 400px;
  box-shadow: var(--shadow-md);
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md);
  border-bottom: 1px solid var(--borderColor);

  h2 {
    margin: 0;
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--textColor);
  }

  .close-button {
    background: transparent;
    border: none;
    color: var(--textSecondary);
    font-size: var(--font-size-2xl);
    cursor: pointer;
    transition: color var(--transition-fast);

    &:hover {
      color: var(--textColor);
    }
  }
}

.modal-body {
  padding: var(--spacing-md);

  .error-message {
    margin-bottom: var(--spacing-md);
    padding: var(--spacing-sm) var(--spacing-md);
    background-color: rgba(220, 38, 38, 0.1);
    color: var(--negativeColor);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-md);
  }

  .form-group {
    margin-bottom: var(--spacing-md);

    label {
      display: block;
      margin-bottom: var(--spacing-sm);
      font-size: var(--font-size-md);
      color: var(--textSecondary);
    }

    input {
      width: 100%;
      padding: var(--spacing-sm) var(--spacing-md);
      border-radius: var(--radius-sm);
      border: 1px solid var(--borderColor);
      background-color: var(--inputBg);
      color: var(--textColor);
      font-size: var(--font-size-md);
      transition: border-color var(--transition-fast);

      &:focus {
        outline: none;
        border-color: var(--accentColor);
      }

      &::placeholder {
        color: var(--textSecondary);
      }

      &:disabled {
        opacity: 0.7;
        cursor: not-allowed;
      }
    }
  }

  .form-actions {
    margin-top: var(--spacing-lg);

    .primary-button {
      width: 100%;
      padding: var(--spacing-sm) var(--spacing-md);
      background-color: var(--buttonBg);
      color: white;
      border: none;
      border-radius: var(--radius-sm);
      font-size: var(--font-size-md);
      font-weight: 500;
      cursor: pointer;
      transition: background-color var(--transition-fast);

      &:hover:not(:disabled) {
        background-color: var(--buttonHoverBg);
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
  padding: var(--spacing-md);
  border-top: 1px solid var(--borderColor);

  .text-button {
    background: transparent;
    border: none;
    color: var(--accentColor);
    font-size: var(--font-size-md);
    cursor: pointer;
    transition: color var(--transition-fast);

    &:hover {
      text-decoration: underline;
    }
  }
}
</style>