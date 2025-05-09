<template>
    <slot></slot>
  </template>
  
  <script setup lang="ts">
  import { ref, provide, onMounted, onUnmounted, watch } from 'vue';
  import { themeService, type ThemeType } from '../services/theme-service';
  
  // 创建主题状态
  const currentTheme = ref<ThemeType>(themeService.getThemePreference());
  
  // 提供给子组件的方法
  const setTheme = (theme: ThemeType) => {
    currentTheme.value = theme;
    themeService.saveThemePreference(theme);
    themeService.applyTheme(theme);
  };
  
  // 监听主题变化
  watch(currentTheme, (newTheme) => {
    themeService.applyTheme(newTheme);
  }, { immediate: true });
  
  // 监听系统主题变化
  let cleanupSystemThemeWatch: (() => void) | null = null;
  
  onMounted(() => {
    // 初始应用主题
    themeService.applyTheme(currentTheme.value);
    
    // 监听系统主题变化
    if (currentTheme.value === 'system') {
      cleanupSystemThemeWatch = themeService.watchSystemTheme(() => {
        // 如果当前是系统主题，则重新应用
        if (currentTheme.value === 'system') {
          themeService.applyTheme('system');
        }
      });
    }
  });
  
  onUnmounted(() => {
    if (cleanupSystemThemeWatch) {
      cleanupSystemThemeWatch();
    }
  });
  
  // 提供主题状态和方法给子组件
  provide('theme', {
    currentTheme,
    setTheme
  });
  </script>