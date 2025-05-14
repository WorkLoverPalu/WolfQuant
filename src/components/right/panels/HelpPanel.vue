<template>
    <div class="custom-panel">
      <!-- 面板内容 -->
      <button @click="$emit('close')">关闭</button>
      <button @click="openInTab">在标签页中打开</button>
    </div>
  </template>
  
  <script setup>
  const emit = defineEmits(['close', 'open-tab']);
  
  const openInTab = () => {
    emit('open-tab', {
      id: 'custom-tab',
      title: '自定义标签',
      component: 'CustomComponent',
      closable: true
    });
  };
  </script>