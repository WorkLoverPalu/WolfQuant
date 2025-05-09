<template>
    <div class="public-info-form">
      <h2 class="section-title">公开资料</h2>
      <p class="section-description">此信息将公开显示，所有用户都可以看到。</p>
      
      <div class="form-group">
        <label for="username">用户名</label>
        <input 
          type="text" 
          id="username" 
          v-model="formData.username" 
          class="form-input"
        />
      </div>
      
      <div class="form-group">
        <label>头像</label>
        <div class="avatar-upload">
          <div class="avatar-preview" :style="{ backgroundColor: '#b79b6c' }">
            {{ formData.avatar }}
          </div>
          <div class="avatar-info">
            <div class="avatar-format">JPG,GIF或者PNG，最大尺寸700KB</div>
            <button class="upload-button">上传图片</button>
            <button class="remove-button">移除图片</button>
          </div>
        </div>
      </div>
      
      <div class="form-group">
        <label class="checkbox-label">
          <input type="checkbox" v-model="formData.showStatus" />
          显示我的在线状态
        </label>
      </div>
      
      <div class="form-group">
        <label>Twitter 账号</label>
        <input 
          type="text" 
          v-model="formData.twitter" 
          class="form-input"
          placeholder="用户名"
        />
      </div>
      
      <div class="form-group">
        <label>YouTube频道链接</label>
        <input 
          type="text" 
          v-model="formData.youtube" 
          class="form-input"
          placeholder="https://www.youtube.com/"
        />
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { reactive, watch } from 'vue';
  
  const props = defineProps({
    userData: {
      type: Object,
      required: true
    }
  });
  
  const emit = defineEmits(['update-user']);
  
  const formData = reactive({
    username: props.userData.username,
    avatar: props.userData.avatar,
    showStatus: true,
    twitter: '',
    youtube: ''
  });
  
  // 监听表单数据变化，自动更新
  watch(formData, (newData) => {
    emit('update-user', { ...newData });
  }, { deep: true });
  </script>
  
  <style lang="scss" scoped>
  .public-info-form {
    background-color: #1e222d;
    border-radius: 8px;
    padding: 20px;
    margin-bottom: 20px;
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
  }
  
  .avatar-upload {
    display: flex;
    align-items: flex-start;
  }
  
  .avatar-preview {
    width: 80px;
    height: 80px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 40px;
    font-weight: bold;
    color: white;
    margin-right: 16px;
  }
  
  .avatar-info {
    flex: 1;
  }
  
  .avatar-format {
    font-size: 12px;
    color: #787b86;
    margin-bottom: 10px;
  }
  
  .upload-button, .remove-button {
    background-color: transparent;
    border: 1px solid #2a2e39;
    color: #d1d4dc;
    padding: 6px 12px;
    border-radius: 4px;
    font-size: 12px;
    cursor: pointer;
    margin-right: 8px;
    
    &:hover {
      background-color: rgba(255, 255, 255, 0.05);
    }
  }
  
  .remove-button {
    color: #787b86;
  }
  
  .checkbox-label {
    display: flex;
    align-items: center;
    cursor: pointer;
    
    input[type="checkbox"] {
      margin-right: 8px;
    }
  }
  </style>