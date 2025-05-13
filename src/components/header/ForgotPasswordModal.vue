<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-container" @click.stop>
      <div class="modal-header">
        <h2>重置密码</h2>
        <button class="close-button" @click="$emit('close')">×</button>
      </div>
      
      <div class="modal-body">
        <div v-if="error" class="error-message">{{ error }}</div>
        <div v-if="success" class="success-message">{{ success }}</div>
        
        <form @submit.prevent="handleSubmit" v-if="!showResetForm">
          <div class="form-group">
            <label for="email">邮箱</label>
            <div class="input-with-button">
              <input 
                type="email" 
                id="email" 
                v-model="email" 
                placeholder="请输入注册邮箱"
                required
                :disabled="isLoading || isCodeSent"
              />
              <button 
                type="button" 
                class="send-code-button"
                @click="sendVerificationCode"
                :disabled="!email || isLoading || countdown > 0"
              >
                {{ countdown > 0 ? `${countdown}s后重试` : '获取验证码' }}
              </button>
            </div>
          </div>
          
          <div class="form-group">
            <label for="verificationCode">验证码</label>
            <input 
              type="text" 
              id="verificationCode" 
              v-model="verificationCode" 
              placeholder="请输入验证码"
              required
              :disabled="isLoading"
            />
          </div>
          
          <div class="form-actions">
            <button type="submit" class="primary-button" :disabled="!email || !verificationCode || isLoading">
              {{ isLoading ? '验证中...' : '验证邮箱' }}
            </button>
          </div>
        </form>
        
        <form v-if="showResetForm" @submit.prevent="handleResetPassword">
          <div class="form-group">
            <label for="newPassword">新密码</label>
            <input 
              type="password" 
              id="newPassword" 
              v-model="newPassword" 
              placeholder="请输入新密码"
              required
              minlength="6"
              :disabled="isResetting"
            />
          </div>
          
          <div class="form-group">
            <label for="confirmNewPassword">确认新密码</label>
            <input 
              type="password" 
              id="confirmNewPassword" 
              v-model="confirmNewPassword" 
              placeholder="请再次输入新密码"
              required
              :disabled="isResetting"
            />
            <p v-if="passwordError" class="field-error">{{ passwordError }}</p>
          </div>
          
          <div class="form-actions">
            <button type="submit" class="primary-button" :disabled="!isResetFormValid || isResetting">
              {{ isResetting ? '重置中...' : '重置密码' }}
            </button>
          </div>
        </form>
      </div>
      
      <div class="modal-footer">
        <button class="text-button" @click="$emit('login')">
          返回登录
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const email = ref('');
const verificationCode = ref('');
const newPassword = ref('');
const confirmNewPassword = ref('');
const isLoading = ref(false);
const isResetting = ref(false);
const isCodeSent = ref(false);
const showResetForm = ref(false);
const countdown = ref(0);
const error = ref('');
const success = ref('');
let countdownTimer: number | null = null;

const passwordError = computed(() => {
  if (newPassword.value && confirmNewPassword.value && newPassword.value !== confirmNewPassword.value) {
    return '两次输入的密码不一致';
  }
  return '';
});

const isResetFormValid = computed(() => {
  return newPassword.value && 
         confirmNewPassword.value && 
         newPassword.value === confirmNewPassword.value;
});

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'login'): void;
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
    await invoke('auth_send_verification_code_command', {
      email: email.value,
      purpose: 'reset_password' // 指定验证码用途
    });
    
    isCodeSent.value = true;
    startCountdown();
    success.value = '验证码已发送，请查收邮箱';
  } catch (err: any) {
    error.value = err.error || '验证码发送失败，请稍后再试';
  } finally {
    isLoading.value = false;
  }
};

const handleSubmit = async () => {
  if (!email.value || !verificationCode.value) return;
  
  isLoading.value = true;
  error.value = '';
  
  try {
    await invoke('verify_reset_password_code', {
      email: email.value,
      code: verificationCode.value
    });
    
    showResetForm.value = true;
    success.value = '验证码正确，请设置新密码';
  } catch (err: any) {
    error.value = err.error || '验证码验证失败，请检查后重试';
  } finally {
    isLoading.value = false;
  }
};

const handleResetPassword = async () => {
  if (!isResetFormValid.value) return;
  
  isResetting.value = true;
  error.value = '';
  
  try {
    const response: any = await invoke('reset_password_command', {
      request: {
        email: email.value,
        code: verificationCode.value,
        new_password: newPassword.value
      }
    });
    
    success.value = response.message;
    
    // 3秒后自动切换到登录页面
    setTimeout(() => {
      emit('login');
    }, 3000);
  } catch (err: any) {
    error.value = err.error || '重置密码失败，请稍后再试';
  } finally {
    isResetting.value = false;
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

  .instruction {
    margin-bottom: 16px;
    color: var(--tab-text);
    font-size: 14px;
    line-height: 1.5;
  }

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

  .reset-token-form {
    margin-top: 16px;

    p {
      margin-bottom: 16px;
      color: var(--tab-text);
      font-size: 14px;
    }
  }
}

.modal-footer {
  display: flex;
  justify-content: center;
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