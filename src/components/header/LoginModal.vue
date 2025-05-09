<template>
    <div class="modal-overlay" @click="$emit('close')">
      <div class="modal-container" @click.stop>
        <div class="modal-header">
          <h2>登录</h2>
          <button class="close-button" @click="$emit('close')">×</button>
        </div>
        
        <div class="modal-body">
          <form @submit.prevent="handleSubmit">
            <div class="form-group">
              <label for="username">用户名</label>
              <input 
                type="text" 
                id="username" 
                v-model="username" 
                placeholder="请输入用户名"
                required
              />
            </div>
            
            <div class="form-group">
              <label for="password">密码</label>
              <input 
                type="password" 
                id="password" 
                v-model="password" 
                placeholder="请输入密码"
                required
              />
            </div>
            
            <div class="form-actions">
              <button type="submit" class="primary-button">登录</button>
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
  
  const username = ref('');
  const password = ref('');
  
  const emit = defineEmits<{
    (e: 'close'): void;
    (e: 'login', username: string, password: string): void;
    (e: 'forgot-password'): void;
    (e: 'register'): void;
  }>();
  
  const handleSubmit = () => {
    if (username.value && password.value) {
      emit('login', username.value, password.value);
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
        
        &:hover {
          background-color: var(--button-primary-hover);
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