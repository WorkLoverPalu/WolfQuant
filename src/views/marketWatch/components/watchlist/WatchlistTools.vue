<template>
    <div class="left-bottom">
      <div class="tools-bar">
        <button class="tool-button">加密货币对游戏器</button>
        <button class="tool-button">Pine编辑器</button>
        <button class="tool-button">策略测试器</button>
        <button class="tool-button">回放交易面板</button>
        <button class="tool-button">交易面板</button>
        
        <div class="tools-right">
          <div class="zoom-control">
            <span>上下拖动缩放</span>
          </div>
          <div class="view-controls">
            <button class="view-button">全屏</button>
            <button class="view-button">隐藏</button>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  // 工具栏组件不需要接收属性
  </script>
  
  <style lang="scss" scoped>
  .left-bottom {
    flex: 1;
    background-color: var(--tools-bg);
    border-top: 1px solid var(--border-color);
    overflow: hidden;
  }
  
  /* 工具栏 */
  .tools-bar {
    display: flex;
    align-items: center;
    padding: 0 12px;
    height: 36px;
    background-color: var(--tools-bg);
    border-bottom: 1px solid var(--border-color);
    overflow-x: auto;
    
    &::-webkit-scrollbar {
      height: 0;
    }
  }
  
  .tool-button {
    padding: 0 12px;
    height: 36px;
    background: transparent;
    border: none;
    color: var(--text-color);
    font-size: 12px;
    white-space: nowrap;
    cursor: pointer;
    
    &:hover {
      background-color: var(--hover-bg);
    }
  }
  
  .tools-right {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 16px;
  }
  
  .zoom-control {
    color: var(--accent-color);
    font-size: 12px;
  }
  
  .view-controls {
    display: flex;
    gap: 8px;
  }
  
  .view-button {
    padding: 0 8px;
    height: 24px;
    background: transparent;
    border: none;
    color: var(--text-color);
    font-size: 12px;
    cursor: pointer;
    
    &:hover {
      text-decoration: underline;
    }
  }
  </style>
  