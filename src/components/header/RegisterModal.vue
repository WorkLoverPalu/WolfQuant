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
const username = ref('nantang');
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

  .success-message {
    margin-bottom: var(--spacing-md);
    padding: var(--spacing-sm) var(--spacing-md);
    background-color: rgba(22, 163, 74, 0.1);
    color: var(--positiveColor);
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

    .field-error {
      margin-top: var(--spacing-xs);
      color: var(--negativeColor);
      font-size: var(--font-size-sm);
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
  justify-content: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-md);
  border-top: 1px solid var(--borderColor);
  color: var(--textSecondary);
  font-size: var(--font-size-md);

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

.input-with-button {
  display: flex;
  gap: var(--spacing-sm);

  input {
    flex: 1;
  }

  .send-code-button {
    padding: 0 var(--spacing-md);
    background-color: var(--buttonBg);
    color: white;
    border: none;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-md);
    cursor: pointer;
    white-space: nowrap;
    transition: background-color var(--transition-fast);

    &:hover:not(:disabled) {
      background-color: var(--buttonHoverBg);
    }

    &:disabled {
      opacity: 0.7;
      cursor: not-allowed;
      background-color: var(--borderColor);
    }
  }
}
</style>