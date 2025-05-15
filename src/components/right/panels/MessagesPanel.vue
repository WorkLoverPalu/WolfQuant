<template>
    <div class="messages-panel">
      <div class="tabs">
        <button 
          class="tab-button" 
          :class="{ active: activeTab === 'public' }"
          @click="activeTab = 'public'"
        >
          公开 <span v-if="unreadPublic" class="tab-badge">{{ unreadPublic }}</span>
        </button>
        <button 
          class="tab-button" 
          :class="{ active: activeTab === 'private' }"
          @click="activeTab = 'private'"
        >
          私密
        </button>
      </div>
      
      <div class="message-actions">
        <div class="action-label">外汇</div>
        <div class="action-buttons">
          <button class="action-button">
            <AtSignIcon />
          </button>
          <button class="action-button">
            <ImageIcon />
          </button>
          <button class="action-button">
            <EditIcon />
          </button>
        </div>
      </div>
      
      <div class="messages-list">
        <div v-for="(message, index) in filteredMessages" :key="index" class="message-item">
          <div class="message-avatar" :style="{ backgroundColor: getAvatarColor(message.user) }">
            {{ message.user.charAt(0).toUpperCase() }}
          </div>
          <div class="message-content">
            <div class="message-header">
              <span class="message-user">{{ message.user }}</span>
              <span class="message-symbol">{{ message.symbol }}</span>
              <span class="message-time">{{ message.time }}</span>
            </div>
            <div class="message-text" v-html="formatMessage(message.text)"></div>
            <div v-if="message.image" class="message-image">
              <img :src="message.image" alt="Chart" />
            </div>
          </div>
        </div>
      </div>
      
      <div class="message-input-container">
        <input 
          type="text" 
          class="message-input" 
          placeholder="有什么要说的话吗?" 
          v-model="newMessage"
          @keyup.enter="sendMessage"
        />
        <button class="send-button" @click="sendMessage">
          <SmileIcon />
        </button>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed } from 'vue';
  import { AtSignIcon, ImageIcon, EditIcon, SmileIcon } from 'lucide-vue-next';
  
  // 定义事件
  const emit = defineEmits<{
    (e: 'close'): void;
  }>();
  
  // 当前激活的标签
  const activeTab = ref('public');
  const unreadPublic = ref(1);
  
  // 新消息输入
  const newMessage = ref('');
  
  // 模拟消息数据
  const messages = ref([
    {
      id: 1,
      user: '@nantang',
      symbol: 'GOLD 15',
      time: '11小时前',
      text: '@aws2333:是滴<br>人家在要理钱啊<br>几秒钟的时差可能是几K钱的差<br>分分钟几百万上下<br>要做男成expert才没有延迟?',
      type: 'public'
    },
    {
      id: 2,
      user: '@nantang',
      symbol: 'ETHUSDTP 30',
      time: '10小时前',
      text: '添加了 止盈止损线段',
      image: 'https://hebbkx1anhila5yf.public.blob.vercel-storage.com/image-QRMgPWc9x5wY3BQeDvZ2sN0P13cFcL.png',
      type: 'public'
    },
    {
      id: 3,
      user: '@nantang',
      symbol: 'TSLA 1',
      time: '38分钟前',
      text: '1.tv编程100行以内100r,300行以内可当天交付<br>2.python定制量化策略，速达到秒，支持电/外交易<br>所<br>欢迎咨询: @nantang',
      type: 'public'
    }
  ]);
  
  // 根据当前标签过滤消息
  const filteredMessages = computed(() => {
    return messages.value.filter(message => message.type === activeTab.value);
  });
  
  // 获取头像颜色
  const getAvatarColor = (username: string) => {
    const colors = ['#e91e63', '#9c27b0', '#673ab7', '#3f51b5', '#2196f3', '#03a9f4', '#00bcd4', '#009688', '#4caf50', '#8bc34a'];
    const index = username.charCodeAt(0) % colors.length;
    return colors[index];
  };
  
  // 格式化消息文本
  const formatMessage = (text: string) => {
    // 已经包含<br>标签，直接返回
    return text;
  };
  
  // 发送消息
  const sendMessage = () => {
    if (!newMessage.value.trim()) return;
    
    messages.value.push({
      id: messages.value.length + 1,
      user: 'current_user',
      symbol: 'BTCUSD',
      time: '刚刚',
      text: newMessage.value,
      type: activeTab.value
    });
    
    newMessage.value = '';
    
    // 消息发送后滚动到底部
    setTimeout(() => {
      const messagesList = document.querySelector('.messages-list');
      if (messagesList) {
        messagesList.scrollTop = messagesList.scrollHeight;
      }
    }, 0);
  };
  </script>
  
  <style lang="scss" scoped>
  .messages-panel {
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  
  .tabs {
    display: flex;
    border-bottom: 1px solid var(--panel-border, #333333);
  }
  
  .tab-button {
    flex: 1;
    padding: 12px;
    background: transparent;
    border: none;
    color: var(--panel-text, #ffffff);
    font-size: 14px;
    cursor: pointer;
    position: relative;
    
    &:hover {
      background-color: rgba(128, 128, 128, 0.05);
    }
    
    &.active {
      font-weight: 500;
      
      &::after {
        content: '';
        position: absolute;
        bottom: -1px;
        left: 0;
        width: 100%;
        height: 2px;
        background-color: var(--icon-active-color, #2563eb);
      }
    }
    
    .tab-badge {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      min-width: 18px;
      height: 18px;
      border-radius: 9px;
      background-color: var(--badge-bg, #e53935);
      color: white;
      font-size: 12px;
      margin-left: 6px;
      padding: 0 4px;
    }
  }
  
  .message-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    border-bottom: 1px solid var(--panel-border, #333333);
  }
  
  .action-label {
    font-size: 14px;
    color: var(--panel-text, #ffffff);
  }
  
  .action-buttons {
    display: flex;
    gap: 8px;
  }
  
  .action-button {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: transparent;
    border: none;
    border-radius: 4px;
    color: var(--icon-color, #a0a0a0);
    cursor: pointer;
    
    &:hover {
      background-color: rgba(128, 128, 128, 0.1);
      color: var(--panel-text, #ffffff);
    }
    
    svg {
      width: 16px;
      height: 16px;
    }
  }
  
  .messages-list {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    
    &::-webkit-scrollbar {
      width: 6px;
    }
    
    &::-webkit-scrollbar-track {
      background: transparent;
    }
    
    &::-webkit-scrollbar-thumb {
      background-color: rgba(128, 128, 128, 0.3);
      border-radius: 3px;
    }
  }
  
  .message-item {
    display: flex;
    margin-bottom: 16px;
    
    &:last-child {
      margin-bottom: 0;
    }
  }
  
  .message-avatar {
    width: 36px;
    height: 36px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 500;
    color: white;
    margin-right: 12px;
    flex-shrink: 0;
  }
  
  .message-content {
    flex: 1;
    min-width: 0;
  }
  
  .message-header {
    display: flex;
    align-items: center;
    margin-bottom: 4px;
    flex-wrap: wrap;
    gap: 6px;
  }
  
  .message-user {
    font-weight: 500;
    color: var(--panel-text, #ffffff);
  }
  
  .message-symbol {
    font-size: 12px;
    color: var(--icon-active-color, #2563eb);
  }
  
  .message-time {
    font-size: 12px;
    color: var(--panel-secondary-text, #a0a0a0);
  }
  
  .message-text {
    font-size: 14px;
    color: var(--panel-text, #ffffff);
    line-height: 1.4;
    word-break: break-word;
  }
  
  .message-image {
    margin-top: 8px;
    max-width: 100%;
    
    img {
      max-width: 100%;
      border-radius: 4px;
    }
  }
  
  .message-input-container {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    border-top: 1px solid var(--panel-border, #333333);
  }
  
  .message-input {
    flex: 1;
    height: 36px;
    padding: 0 12px;
    background-color: var( --inputBg, #2c2c2c);
    border: 1px solid var(--panel-border, #333333);
    border-radius: 18px;
    color: var(--panel-text, #ffffff);
    font-size: 14px;
    
    &:focus {
      outline: none;
      border-color: var(--icon-active-color, #2563eb);
    }
    
    &::placeholder {
      color: var(--panel-secondary-text, #a0a0a0);
    }
  }
  
  .send-button {
    width: 36px;
    height: 36px;
    margin-left: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: transparent;
    border: none;
    border-radius: 50%;
    color: var(--icon-color, #a0a0a0);
    cursor: pointer;
    
    &:hover {
      background-color: rgba(128, 128, 128, 0.1);
      color: var(--icon-active-color, #2563eb);
    }
    
    svg {
      width: 20px;
      height: 20px;
    }
  }
  </style>