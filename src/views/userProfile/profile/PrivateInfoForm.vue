<template>
    <div class="private-info-form">
      <h2 class="section-title">私密资料</h2>
      <p class="section-description">此信息不会公开显示。</p>
      
      <div class="form-group">
        <label for="email">电子邮箱</label>
        <div class="email-group">
          <input 
            type="email" 
            id="email" 
            v-model="emailValue" 
            class="form-input"
            disabled
          />
          <button class="update-button" @click="showEmailModal = true">
            更改电子邮箱
          </button>
        </div>
      </div>
      
      <div class="form-group">
        <label for="password">密码</label>
        <button class="update-button" @click="showPasswordModal = true">
          修改密码
        </button>
      </div>
      
      <div class="form-group">
        <div class="security-note">
          让您的账户更安全
          <button class="security-button" @click="$emit('enable-2fa')">
            设置双因素验证
          </button>
        </div>
      </div>
      
      <div class="form-group">
        <label>电话</label>
        <div class="phone-group">
          <input 
            type="tel" 
            v-model="phoneValue" 
            class="form-input"
            placeholder="添加您的手机号码"
          />
          <button class="add-button" v-if="!phoneValue">
            + 添加您的手机号码
          </button>
        </div>
      </div>
      
      <div class="form-group">
        <label>备用的电子邮箱</label>
        <div class="backup-email-group">
          <button class="add-button">
            + 添加电子邮件
          </button>
        </div>
        <p class="helper-text">可用于发送重要的电子邮件通知。</p>
      </div>
      
      <div class="form-actions">
        <button class="save-button" @click="saveChanges">
          保存更改
        </button>
      </div>
      
      <!-- 修改邮箱模态框 -->
      <div class="modal" v-if="showEmailModal">
        <div class="modal-content">
          <h3>更改电子邮箱</h3>
          <div class="form-group">
            <label for="new-email">新电子邮箱</label>
            <input 
              type="email" 
              id="new-email" 
              v-model="newEmail" 
              class="form-input"
            />
          </div>
          <div class="modal-actions">
            <button class="cancel-button" @click="showEmailModal = false">取消</button>
            <button class="confirm-button" @click="updateEmail">确认</button>
          </div>
        </div>
      </div>
      
      <!-- 修改密码模态框 -->
      <div class="modal" v-if="showPasswordModal">
        <div class="modal-content">
          <h3>修改密码</h3>
          <div class="form-group">
            <label for="current-password">当前密码</label>
            <input 
              type="password" 
              id="current-password" 
              v-model="currentPassword" 
              class="form-input"
            />
          </div>
          <div class="form-group">
            <label for="new-password">新密码</label>
            <input 
              type="password" 
              id="new-password" 
              v-model="newPassword" 
              class="form-input"
            />
          </div>
          <div class="form-group">
            <label for="confirm-password">确认新密码</label>
            <input 
              type="password" 
              id="confirm-password" 
              v-model="confirmPassword" 
              class="form-input"
            />
          </div>
          <div class="modal-actions">
            <button class="cancel-button" @click="showPasswordModal = false">取消</button>
            <button class="confirm-button" @click="updatePassword">确认</button>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref } from 'vue';
  
  const props = defineProps({
    email: {
      type: String,
      required: true
    }
  });
  
  const emit = defineEmits(['update-email', 'update-password', 'enable-2fa']);
  
  const emailValue = ref(props.email);
  const phoneValue = ref('');
  
  // 邮箱修改
  const showEmailModal = ref(false);
  const newEmail = ref('');
  
  const updateEmail = () => {
    if (newEmail.value) {
      emailValue.value = newEmail.value;
      emit('update-email', newEmail.value);
      showEmailModal.value = false;
      newEmail.value = '';
    }
  };
  
  // 密码修改
  const showPasswordModal = ref(false);
  const currentPassword = ref('');
  const newPassword = ref('');
  const confirmPassword = ref('');
  
  const updatePassword = () => {
    if (currentPassword.value && newPassword.value && newPassword.value === confirmPassword.value) {
      emit('update-password', newPassword.value);
      showPasswordModal.value = false;
      currentPassword.value = '';
      newPassword.value = '';
      confirmPassword.value = '';
    }
  };
  
  // 保存更改
  const saveChanges = () => {
    // 实际应用中这里会调用API保存所有更改
    console.log('保存更改');
  };
  </script>
  
  <style lang="scss" scoped>
  .private-info-form {
    background-color: #1e222d;
    border-radius: 8px;
    padding: 20px;
    position: relative;
  }
  
  .section-title {
    font-size: 18px;
    font-weight: 500;
    margin-bottom: 8px;
    color: #d1d4dc;
  }
  
  .section-description {
    font-size: 14px;
    color: #787b86;
    margin-bottom: 20px;
  }
  
  .form-group {
    margin-bottom: 20px;
    
    label {
      display: block;
      font-size: 14px;
      color: #d1d4dc;
      margin-bottom: 8px;
    }
  }
  
  .form-input {
    width: 100%;
    padding: 10px;
    background-color: #131722;
    border: 1px solid #2a2e39;
    border-radius: 4px;
    color: #d1d4dc;
    font-size: 14px;
    
    &:focus {
      outline: none;
      border-color: #2962ff;
    }
    
    &::placeholder {
      color: #5d606b;
    }
    
    &:disabled {
      opacity: 0.7;
      cursor: not-allowed;
    }
  }
  
  .email-group, .phone-group, .backup-email-group {
    display: flex;
    align-items: center;
    
    .form-input {
      flex: 1;
      margin-right: 10px;
    }
  }
  
  .update-button, .add-button {
    background-color: transparent;
    border: 1px solid #2a2e39;
    color: #2962ff;
    padding: 8px 12px;
    border-radius: 4px;
    font-size: 14px;
    cursor: pointer;
    white-space: nowrap;
    
    &:hover {
      background-color: rgba(41, 98, 255, 0.1);
    }
  }
  
  .security-note {
    display: flex;
    align-items: center;
    font-size: 14px;
    color: #787b86;
  }
  
  .security-button {
    background-color: transparent;
    border: none;
    color: #2962ff;
    margin-left: 8px;
    cursor: pointer;
    font-size: 14px;
    
    &:hover {
      text-decoration: underline;
    }
  }
  
  .helper-text {
    font-size: 12px;
    color: #787b86;
    margin-top: 4px;
  }
  
  .form-actions {
    margin-top: 30px;
    text-align: right;
  }
  
  .save-button {
    background-color: #2962ff;
    border: none;
    color: white;
    padding: 8px 16px;
    border-radius: 4px;
    font-size: 14px;
    cursor: pointer;
    
    &:hover {
      background-color: #1e53e4;
    }
  }
  
  .modal {
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
  
  .modal-content {
    background-color: #1e222d;
    border-radius: 8px;
    padding: 20px;
    width: 400px;
    max-width: 90%;
    
    h3 {
      font-size: 18px;
      margin-bottom: 20px;
      color: #d1d4dc;
    }
  }
  
  .modal-actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 20px;
    gap: 10px;
  }
  
  .cancel-button {
    background-color: transparent;
    border: 1px solid #2a2e39;
    color: #d1d4dc;
    padding: 8px 16px;
    border-radius: 4px;
    font-size: 14px;
    cursor: pointer;
    
    &:hover {
      background-color: rgba(255, 255, 255, 0.05);
    }
  }
  
  .confirm-button {
    background-color: #2962ff;
    border: none;
    color: white;
    padding: 8px 16px;
    border-radius: 4px;
    font-size: 14px;
    cursor: pointer;
    
    &:hover {
      background-color: #1e53e4;
    }
  }
  </style>