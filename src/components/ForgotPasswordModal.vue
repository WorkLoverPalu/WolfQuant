<template>
    <div class="modal-overlay" @click="$emit('close')">
      <div class="modal-container" @click.stop>
        <div class="modal-header">
          <h2>重置密码</h2>
          <button class="close-button" @click="$emit('close')">×</button>
        </div>
        
        <div class="modal-body">
          <p class="instruction">请输入您的注册邮箱，我们将向您发送密码重置链接。</p>
          
          <form @submit.prevent="handleSubmit">
            <div class="form-group">
              <label for="email">邮箱</label>
              <input 
                type="email" 
                id="email" 
                v-model="email" 
                placeholder="请输入注册邮箱"
                required
              />
            </div>
            
            <div class="form-actions">
              <button type="submit" class="primary-button" :disabled="!email">
                发送重置链接
              </button>
            </div>
          </form>
        </div>
        
        <div class="modal-footer">
          <button class="text-button" @click="$emit('close')">
            返回登录
          </button>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref } from 'vue';
  
  const email = ref('');
  
  const emit = defineEmits<{
    (e: 'close'): void;
    (e: 'reset-password', email: string): void;
  }>();
  
  const handleSubmit = () => {
    if (email.value) {
      emit('reset-password', email.value);
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
          opacity: 0.6;
          cursor: not-allowed;
        }
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