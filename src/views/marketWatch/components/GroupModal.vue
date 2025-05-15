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
          <input type="text" id="groupName" class="form-input" v-model="groupForm.name" placeholder="请输入分组名称" />
        </div>

        <div class="form-group">
          <label for="assetType">资产类型</label>
          <select id="assetType" class="form-select" v-model="groupForm.assetTypeId">
            <option v-for="type in assetStore.assetTypes" :key="type.id" :value="type.id">
              {{ type.name }}
            </option>
          </select>
        </div>

        <div class="form-group">
          <label for="groupDescription">描述 (可选)</label>
          <input type="text" id="groupDescription" class="form-input" v-model="groupForm.description" placeholder="请输入分组描述" />
        </div>
      </div>

      <div class="modal-footer">
        <button class="cancel-button" @click="closeModal" :disabled="loading">取消</button>
        <button class="confirm-button" @click="saveGroup" :disabled="loading">
          <span v-if="loading">处理中...</span>
          <span v-else>确定</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, defineProps, defineEmits } from 'vue';
import { XIcon } from 'lucide-vue-next';
import { useAssetStore, } from '../../../stores/assetStore';
import { useUserStore } from '../../../stores/userStore';
import type { UserGroup } from '../../../stores/assetStore';

const props = defineProps<{
  show: boolean;
  editingGroup?: UserGroup | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'saved', group: UserGroup): void;
}>();

const assetStore = useAssetStore();
const userStore = useUserStore();

// 表单状态
const groupForm = ref({
  name: '',
  assetTypeId: 0,
  description: ''
});

const loading = ref(false);
const error = ref<string | null>(null);

// 初始化
onMounted(async () => {
  // 加载资产类型
  if (assetStore.assetTypes.length === 0) {
    try {
      await assetStore.fetchAssetTypes();
      
      // 设置默认资产类型
      if (assetStore.assetTypes.length > 0 && !groupForm.value.assetTypeId) {
        groupForm.value.assetTypeId = assetStore.assetTypes[0].id;
      }
    } catch (err) {
      console.error('Failed to load asset types:', err);
      error.value = '加载资产类型失败';
    }
  }
  
  // 如果是编辑模式，填充表单
  if (props.editingGroup) {
    groupForm.value = {
      name: props.editingGroup.name,
      assetTypeId: props.editingGroup.asset_type_id,
      description: props.editingGroup.description || ''
    };
  } else if (assetStore.assetTypes.length > 0) {
    // 新建模式，设置默认资产类型
    groupForm.value.assetTypeId = assetStore.assetTypes[0].id;
  }
});

// 关闭模态框
const closeModal = () => {
  emit('close');
};

// 保存分组
const saveGroup = async () => {
  if (!groupForm.value.name.trim()) {
    error.value = '请输入分组名称';
    return;
  }
  
  if (!groupForm.value.assetTypeId) {
    error.value = '请选择资产类型';
    return;
  }
  
  loading.value = true;
  error.value = null;
  
  try {
    let savedGroup: UserGroup;
    
    if (props.editingGroup) {
      // 更新分组
      savedGroup = await assetStore.updateUserGroup(
        props.editingGroup.id,
        groupForm.value.name,
        groupForm.value.description || undefined
      );
    } else {
      // 创建新分组
      savedGroup = await assetStore.createUserGroup(
        groupForm.value.name,
        groupForm.value.assetTypeId,
        groupForm.value.description || undefined
      );
    }
    
    emit('saved', savedGroup);
    closeModal();
  } catch (err) {
    console.error('Failed to save group:', err);
    error.value = props.editingGroup ? '更新分组失败' : '创建分组失败';
  } finally {
    loading.value = false;
  }
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
  background-color: var(--modalBg);
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
  border-bottom: 1px solid var(--borderColor);

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
    color: var(--textSecondary);
    cursor: pointer;

    &:hover {
      background-color: var(--hover-bg);
      color: var(--textColor);
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

.form-input,
.form-select {
  width: 100%;
  height: 40px;
  padding: 0 12px;
  background-color: var( --inputBg);
  border: 1px solid var(--borderColor);
  border-radius: 4px;
  color: var(--textColor);
  font-size: 14px;

  &:focus {
    outline: none;
    border-color: var(--accentColor);
  }
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px;
  border-top: 1px solid var(--borderColor);
}

.cancel-button {
  padding: 8px 16px;
  background-color: transparent;
  border: 1px solid var(--borderColor);
  border-radius: 4px;
  color: var(--textColor);
  font-size: 14px;
  cursor: pointer;

  &:hover:not(:disabled) {
    background-color: var(--hover-bg);
  }
  
  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.confirm-button {
  padding: 8px 16px;
  background-color: var(--buttonBg);
  border: none;
  border-radius: 4px;
  color: white;
  font-size: 14px;
  cursor: pointer;

  &:hover:not(:disabled) {
    background-color: var( --buttonHoverBg);
  }
  
  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.error-message {
  color: #ef4444;
  font-size: 14px;
  margin-top: 8px;
}
</style>