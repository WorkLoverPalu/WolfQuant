<template>
  <slot></slot>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, provide } from 'vue';
import { useThemeStore } from '../stores/themeStore';

// 使用 Pinia theme store
const themeStore = useThemeStore();

// 监听系统主题变化
let cleanupSystemThemeWatch: (() => void) | null = () => {};

onMounted(() => {
  // 初始化主题
  themeStore.initTheme();
});

onUnmounted(() => {
  if (cleanupSystemThemeWatch) {
    cleanupSystemThemeWatch();
  }
});

// 为了向后兼容，仍然提供主题状态和方法给子组件
// 这样现有的使用 inject('theme') 的组件不需要修改
provide('theme', {
  currentTheme: themeStore.currentTheme,
  setTheme: themeStore.setTheme
});
</script>