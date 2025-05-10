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
        
        <p v-if="!success" class="instruction">请输入您的注册邮箱，我们将向您发送密码重置链接。</p>
        
        <form @submit.prevent="handleSubmit" v-if="!success">
          <div class="form-group">
            <label for="email">邮箱</label>
            <input 
              type="email" 
              id="email" 
              v-model="email" 
              placeholder="请输入注册邮箱"
              required
              :disabled="isLoading"
            />
          </div>
          
          <div class="form-actions">
            <button type="submit" class="primary-button" :disabled="!email || isLoading">
              {{ isLoading ? '发送中...' : '发送重置链接' }}
            </button>
          </div>
        </form>
        
        <div v-if="success" class="reset-token-form">
          <p>请输入重置令牌和新密码：</p>
          
          <form @submit.prevent="handleResetPassword">
            <div class="form-group">
              <label for="token">重置令牌</label>
              <input 
                type="text" 
                id="token" 
                v-model="token" 
                placeholder="请输入重置令牌"
                required
                :disabled="isResetting"
              />
            </div>
            
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
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const email = ref('');
const token = ref('');
const newPassword = ref('');
const confirmNewPassword = ref('');
const isLoading = ref(false);
const isResetting = ref(false);
const error = ref('');
const success = ref('');

const passwordError = computed(() => {
  if (newPassword.value && confirmNewPassword.value && newPassword.value !== confirmNewPassword.value) {
    return '两次输入的密码不一致';
  }
  return '';
});

const isResetFormValid = computed(() => {
  return token.value && 
         newPassword.value && 
         confirmNewPassword.value && 
         newPassword.value === confirmNewPassword.value;
});

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'login'): void;
}>();

const handleSubmit = async () => {
  if (!email.value) return;
  
  isLoading.value = true;
  error.value = '';
  
  try {
    const response: any = await invoke('forgot_password', {
      request: {
        email: email.value
      }
    });
    
    success.value = response.message;
  } catch (err: any) {
    error.value = err.error || '发送重置链接失败，请稍后再试';
    success.value = '';
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
        token: token.value,
        new_password: newPassword.value
      }
    });
    
    success.value = response.message;
    
    // 清空表单
    token.value = '';
    newPassword.value = '';
    confirmNewPassword.value = '';
    
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
</style>