// 主题类型
export type ThemeType = 'system' | 'dark' | 'light';

// 主题颜色变量
interface ThemeColors {
  bgColor: string;
  headerBg: string;
  tabBg: string;
  tabActiveBg: string;
  tabText: string;
  tabActiveText: string;
  borderColor: string;
  avatarBg: string;
  avatarText: string;
  modalBg: string;
  inputBg: string;
  buttonPrimary: string;
  buttonPrimaryHover: string;
}

// 定义不同主题的颜色
const darkTheme: ThemeColors = {
  bgColor: '#121212',
  headerBg: '#1a1a1a',
  tabBg: '#252525',
  tabActiveBg: '#2a2a2a',
  tabText: '#a0a0a0',
  tabActiveText: '#ffffff',
  borderColor: '#333333',
  avatarBg: '#4a4a4a',
  avatarText: '#ffffff',
  modalBg: '#1e1e1e',
  inputBg: '#2c2c2c',
  buttonPrimary: '#2563eb',
  buttonPrimaryHover: '#3b82f6'
};

const lightTheme: ThemeColors = {
  bgColor: '#f5f5f5',
  headerBg: '#ffffff',
  tabBg: '#e5e5e5',
  tabActiveBg: '#ffffff',
  tabText: '#666666',
  tabActiveText: '#000000',
  borderColor: '#dddddd',
  avatarBg: '#e0e0e0',
  avatarText: '#333333',
  modalBg: '#ffffff',
  inputBg: '#f0f0f0',
  buttonPrimary: '#2563eb',
  buttonPrimaryHover: '#3b82f6'
};

// 获取系统主题偏好
const getSystemTheme = (): 'dark' | 'light' => {
  return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches
    ? 'dark'
    : 'light';
};

// 监听系统主题变化
const watchSystemTheme = (callback: (theme: 'dark' | 'light') => void) => {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  
  const handleChange = (e: MediaQueryListEvent) => {
    callback(e.matches ? 'dark' : 'light');
  };
  
  // 添加监听器
  if (mediaQuery.addEventListener) {
    mediaQuery.addEventListener('change', handleChange);
  } else {
    // 兼容旧版浏览器
    mediaQuery.addListener(handleChange);
  }
  
  // 返回清理函数
  return () => {
    if (mediaQuery.removeEventListener) {
      mediaQuery.removeEventListener('change', handleChange);
    } else {
      // 兼容旧版浏览器
      mediaQuery.removeListener(handleChange);
    }
  };
};

// 保存主题到本地存储
const saveThemePreference = (theme: ThemeType) => {
  localStorage.setItem('theme-preference', theme);
};

// 从本地存储获取主题
const getThemePreference = (): ThemeType => {
  return (localStorage.getItem('theme-preference') as ThemeType) || 'system';
};

// 应用主题到文档
const applyTheme = (theme: ThemeType) => {
  const actualTheme = theme === 'system' ? getSystemTheme() : theme;
  const colors = actualTheme === 'dark' ? darkTheme : lightTheme;
  
  // 设置CSS变量
  const root = document.documentElement;
  
  Object.entries(colors).forEach(([key, value]) => {
    // 将驼峰命名转换为CSS变量命名
    const cssVarName = key.replace(/([A-Z])/g, '-$1').toLowerCase();
    root.style.setProperty(`--${cssVarName}`, value);
  });
  
  // 设置数据属性，用于CSS选择器
  document.body.dataset.theme = actualTheme;
};

export const themeService = {
  getSystemTheme,
  watchSystemTheme,
  saveThemePreference,
  getThemePreference,
  applyTheme
};