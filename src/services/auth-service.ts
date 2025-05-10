import { invoke } from '@tauri-apps/api/tauri';

export interface User {
  id: string;
  username: string;
  email: string;
  created_at: number;
  updated_at: number;
}

export interface AuthResponse {
  user: User;
  message: string;
  token?: string;
}

export interface MessageResponse {
  message: string;
}

// 登录
export async function login(username: string, password: string): Promise<AuthResponse> {
  try {
    const response = await invoke<AuthResponse>('login_command', {
      request: {
        username_or_email: username,
        password: password
      }
    });
    
    // 保存会话信息
    if (response.token) {
      localStorage.setItem('auth_token', response.token);
      localStorage.setItem('user', JSON.stringify(response.user));
    }
    
    return response;
  } catch (error) {
    console.error('Login error:', error);
    throw error;
  }
}

// 注册
export async function register(email: string, password: string, username: string): Promise<AuthResponse> {
  try {
    const response = await invoke<AuthResponse>('register_command', {
      request: {
        username,
        email,
        password
      }
    });
    
    return response;
  } catch (error) {
    console.error('Registration error:', error);
    throw error;
  }
}

// 忘记密码
export async function forgotPassword(email: string): Promise<MessageResponse> {
  try {
    const response = await invoke<MessageResponse>('forgot_password_command', {
      request: {
        email
      }
    });
    
    return response;
  } catch (error) {
    console.error('Forgot password error:', error);
    throw error;
  }
}

// 重置密码
export async function resetPassword(token: string, newPassword: string): Promise<MessageResponse> {
  try {
    const response = await invoke<MessageResponse>('reset_password_command', {
      request: {
        token,
        new_password: newPassword
      }
    });
    
    return response;
  } catch (error) {
    console.error('Reset password error:', error);
    throw error;
  }
}

// 登出
export async function logout(): Promise<MessageResponse> {
  try {
    const user = getCurrentUser();
    const token = localStorage.getItem('auth_token');
    
    if (!user || !token) {
      throw new Error('未登录');
    }
    
    const response = await invoke<MessageResponse>('logout_command', {
      request: {
        user_id: user.id,
        token
      }
    });
    
    // 清除本地存储
    localStorage.removeItem('auth_token');
    localStorage.removeItem('user');
    
    return response;
  } catch (error) {
    console.error('Logout error:', error);
    throw error;
  }
}

// 验证会话
export async function verifySession(): Promise<User | null> {
  try {
    const token = localStorage.getItem('auth_token');
    
    if (!token) {
      return null;
    }
    
    const user = await invoke<User>('verify_session_command', {
      request: {
        token
      }
    });
    
    // 更新本地存储的用户信息
    localStorage.setItem('user', JSON.stringify(user));
    
    return user;
  } catch (error) {
    console.error('Session verification error:', error);
    // 清除无效的会话
    localStorage.removeItem('auth_token');
    localStorage.removeItem('user');
    return null;
  }
}

// 获取当前用户
export function getCurrentUser(): User | null {
  const userJson = localStorage.getItem('user');
  
  if (!userJson) {
    return null;
  }
  
  try {
    return JSON.parse(userJson) as User;
  } catch {
    return null;
  }
}

// 检查是否已登录
export function isLoggedIn(): boolean {
  return !!localStorage.getItem('auth_token') && !!getCurrentUser();
}