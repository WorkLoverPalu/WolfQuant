<template>
    <div class="modal-overlay" @click="closeModal">
      <div class="modal-container group-modal" @click.stop>
        <div class="modal-header">
          <h3>{{ editingGroup ? '编辑分组' : '添加分组' }}</h3>
          <button class="close-button" @click="closeModal">
            <XIcon />
          </button>
        </div>
        
        <div class="group-form">
          <div class="form-group">
            <label for="groupName">分组名称</label>
            <input 
              type="text" 
              id="groupName" 
              class="form-input" 
              v-model="groupForm.name"
              placeholder="请输入分组名称"
            />
          </div>
          
          <div class="form-group">
            <label for="groupCategory">所属分类</label>
            <select 
              id="groupCategory" 
              class="form-select" 
              v-model="groupForm.category"
            >
              <option v-for="category in categories" :key="category.id" :value="category.id">
                {{ category.name }}
              </option>
            </select>
          </div>
        </div>
        
        <div class="modal-footer">
          <button class="cancel-button" @click="closeModal">取消</button>
          <button class="confirm-button" @click="saveGroup">确定</button>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, inject, onMounted } from 'vue';
  import { XIcon } from 'lucide-vue-next';
  
  // 注入全局状态
  const showGroupModal = inject('showGroupModal');
  const activeCategory = inject('activeCategory');
  const groups = inject('groups');
  
  // 分类数据
  const categories = [
    { id: 'fund', name: '基金' },
    { id: 'stock', name: '股票' },
    { id: 'gold', name: '黄金' },
    { id: 'crypto', name: '数字货币' }
  ];
  
  // 编辑状态
  const editingGroup = ref(null);
  const groupForm = ref({
    name: '',
    category: activeCategory.value
  });
  
  // 关闭模态框
  const closeModal = () => {
    showGroupModal.value = false;
    editingGroup.value = null;
    groupForm.value = {
      name: '',
      category: activeCategory.value
    };
  };
  
  // 保存分组
  const saveGroup = () => {
    if (!groupForm.value.name.trim()) {
      alert('请输入分组名称');
      return;
    }
    
    if (editingGroup.value) {
      // 更新现有分组
      const group = groups.value.find(g => g.id === editingGroup.value.id);
      if (group) {
        group.name = groupForm.value.name;
        group.category = groupForm.value.category;
      }
    } else {
      // 添加新分组
      const newGroupId = 'group_' + Date.now();
      groups.value.push({
        id: newGroupId,
        name: groupForm.value.name,
        category: groupForm.value.category,
        items: []
      });
    }
    
    closeModal();
  };
  </script>
  
  <style lang="scss" scoped>
  /* 弹窗样式 */
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
    width: 90%;
    max-width: 600px;
    max-height: 80vh;
    background-color: var(--modal-bg);
    border-radius: 8px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    
    &.group-modal {
      max-width: 400px;
    }
  }
  
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    border-bottom: 1px solid var(--border-color);
    
    h3 {
      margin: 0;
      font-size: 18px;
      font-weight: 500;
    }
    
    .close-button {
      width: 32px;
      height: 32px;
      border-radius: 4px;
      display: flex;
      align-items: center;
      justify-content: center;
      background: transparent;
      border: none;
      color: var(--text-secondary);
      cursor: pointer;
      
      &:hover {
        background-color: var(--hover-bg);
        color: var(--text-color);
      }
      
      svg {
        width: 20px;
        height: 20px;
      }
    }
  }
  
  /* 分组表单 */
  .group-form {
    padding: 16px;
  }
  
  .form-group {
    margin-bottom: 16px;
    
    &:last-child {
      margin-bottom: 0;
    }
    
    label {
      display: block;
      margin-bottom: 8px;
      font-size: 14px;
    }
  }
  
  .form-input, .form-select {
    width: 100%;
    height: 40px;
    padding: 0 12px;
    background-color: var(--input-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-color);
    font-size: 14px;
    
    &:focus {
      outline: none;
      border-color: var(--accent-color);
    }
  }
  
  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding: 16px;
    border-top: 1px solid var(--border-color);
  }
  
  .cancel-button {
    padding: 8px 16px;
    background-color: transparent;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-color);
    font-size: 14px;
    cursor: pointer;
    
    &:hover {
      background-color: var(--hover-bg);
    }
  }
  
  .confirm-button {
    padding: 8px 16px;
    background-color: var(--button-bg);
    border: none;
    border-radius: 4px;
    color: white;
    font-size: 14px;
    cursor: pointer;
    
    &:hover {
      background-color: var(--button-hover-bg);
    }
  }
  </style>
  