<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-container" @click.stop>
      <div class="modal-header">
        <h2>注册账号</h2>
        <button class="close-button" @click="$emit('close')">×</button>
      </div>

      <div class="modal-body">
        <div v-if="error" class="error-message">{{ error }}</div>
        <div v-if="success" class="success-message">{{ success }}</div>

        <form @submit.prevent="handleSubmit" v-if="!success">
          <div class="form-group">
            <label for="email">邮箱</label>
            <div class="input-with-button">
              <input type="email" id="email" v-model="email" placeholder="请输入邮箱" required
                :disabled="isLoading || isCodeSent" />
              <button type="button" class="send-code-button" @click="sendVerificationCode"
                :disabled="!email || isLoading || countdown > 0">
                {{ countdown > 0 ? `${countdown}s后重试` : '获取验证码' }}
              </button>
            </div>
          </div>

          <div class="form-group">
            <label for="verificationCode">验证码</label>
            <input type="text" id="verificationCode" v-model="verificationCode" placeholder="请输入验证码" required
              :disabled="isLoading" />
          </div>

          <div class="form-group">
            <label for="username">用户名</label>
            <input type="text" id="username" v-model="username" placeholder="请输入用户名" required :disabled="isLoading" />
          </div>

          <div class="form-group">
            <label for="password">密码</label>
            <input type="password" id="password" v-model="password" placeholder="请设置密码" required minlength="6"
              :disabled="isLoading" />
          </div>

          <div class="form-group">
            <label for="confirmPassword">确认密码</label>
            <input type="password" id="confirmPassword" v-model="confirmPassword" placeholder="请再次输入密码" required
              :disabled="isLoading" />
            <p v-if="passwordError" class="field-error">{{ passwordError }}</p>
          </div>

          <div class="form-actions">
            <button type="submit" class="primary-button" :disabled="!isFormValid || isLoading">
              {{ isLoading ? '注册中...' : '注册' }}
            </button>
          </div>
        </form>
      </div>

      <div class="modal-footer">
        <span>已有账号？</span>
        <button class="text-button" @click="$emit('login')">
          返回登录
        </button>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue';
import { useUserStore } from '../../stores/userStore';

const email = ref('123@qq.com');
const verificationCode = ref('');
const username = ref('natang');
const password = ref('qwe123');
const confirmPassword = ref('qwe123');
const isLoading = ref(false);
const isCodeSent = ref(false);
const countdown = ref(0);
const error = ref('');
const success = ref('');
let countdownTimer: number | null = null;

const userStore = useUserStore();

const passwordError = computed(() => {
  if (password.value && confirmPassword.value && password.value !== confirmPassword.value) {
    return '两次输入的密码不一致';
  }
  return '';
});

const isFormValid = computed(() => {
  return email.value &&
    verificationCode.value &&
    username.value &&
    password.value &&
    confirmPassword.value &&
    password.value === confirmPassword.value;
});

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'login'): void;
  (e: 'login-success', user: any, token: any): void;
}>();

const startCountdown = () => {
  countdown.value = 60; // 60秒倒计时
  countdownTimer = setInterval(() => {
    countdown.value--;
    if (countdown.value <= 0 && countdownTimer) {
      clearInterval(countdownTimer);
      countdownTimer = null;
    }
  }, 1000) as unknown as number;
};

const sendVerificationCode = async () => {
  if (!email.value) return;

  isLoading.value = true;
  error.value = '';

  try {
    // 使用 store 的 sendVerificationCode 方法
    await userStore.sendVerificationCode(email.value, "register");

    isCodeSent.value = true;
    startCountdown();
  } catch (err: any) {
    error.value = err.message || '验证码发送失败，请稍后再试';
  } finally {
    isLoading.value = false;
  }
};

const handleSubmit = async () => {
  if (!isFormValid.value) return;

  isLoading.value = true;
  error.value = '';
  success.value = '';

  try {
    // 使用 store 的 register 方法
    const result = await userStore.register(
      username.value, 
      email.value, 
      password.value, 
      verificationCode.value
    );

    success.value = result.message;

    // 清空表单
    email.value = '';
    verificationCode.value = '';
    username.value = '';
    password.value = '';
    confirmPassword.value = '';
    isCodeSent.value = false;

    // 1秒后自动切换到登录页面
    setTimeout(() => {
      emit('login-success', result.user, userStore.token);
    }, 1000);
  } catch (err: any) {
    error.value = err.message || '注册失败，请稍后再试';
  } finally {
    isLoading.value = false;
  }
};

onUnmounted(() => {
  if (countdownTimer) {
    clearInterval(countdownTimer);
  }
});
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

  .success-message {
    margin-bottom: 16px;
    padding: 10px;
    background-color: rgba(22, 163, 74, 0.1);
    color: #16a34a;
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

    .field-error {
      margin-top: 4px;
      color: #dc2626;
      font-size: 12px;
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
  justify-content: center;
  gap: 8px;
  padding: 16px;
  border-top: 1px solid var(--border-color);
  color: var(--tab-text);
  font-size: 14px;

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

.input-with-button {
  display: flex;
  gap: 8px;

  input {
    flex: 1;
  }

  .send-code-button {
    padding: 0 12px;
    background-color: var(--button-primary);
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 14px;
    cursor: pointer;
    white-space: nowrap;

    &:hover:not(:disabled) {
      background-color: var(--button-primary-hover);
    }

    &:disabled {
      opacity: 0.7;
      cursor: not-allowed;
      background-color: var(--border-color);
    }
  }
}
</style>