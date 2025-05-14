<template>
    <div class="user-profile-container">
      <div class="profile-content">
        <!-- 左侧用户资料部分 -->
        <div class="profile-main">
          <!-- 用户头部信息 -->
          <UserProfileHeader 
            :username="userData.username" 
            :avatar="userData.avatar" 
            :isOnline="userData.isOnline"
            :joinDate="userData.joinDate"
          />
          
          <!-- 用户统计数据 -->
          <UserStats 
            :followers="userData.stats.followers"
            :following="userData.stats.following"
            :views="userData.stats.views"
            :scripts="userData.stats.scripts"
          />
          
          <!-- 导航标签 -->
          <ProfileTabs 
            :activeTab="activeTab"
            @change-tab="activeTab = $event"
          />
          
          <!-- 设置子标签 (仅在设置标签激活时显示) -->
          <SettingsTabs 
            v-if="activeTab === 'settings'"
            :activeSettingsTab="activeSettingsTab"
            @change-tab="activeSettingsTab = $event"
          />
          
          <!-- 内容区域 -->
          <div class="profile-content-area">
            <!-- 根据当前激活的标签显示不同内容 -->
            <div v-if="activeTab === 'settings'">
              <div v-if="activeSettingsTab === 'general'" class="settings-forms">
                <!-- 公开资料表单 -->
                <PublicInfoForm 
                  :userData="userData"
                  @update-user="updateUserData"
                />
                
                <!-- 私密资料表单 -->
                <PrivateInfoForm 
                  :email="userData.email"
                  @update-email="updateEmail"
                  @update-password="updatePassword"
                />
              </div>
              
              <div v-else-if="activeSettingsTab === 'account'" class="settings-forms">
                <!-- 账户设置内容 -->
                <AccountSettings />
              </div>
              
              <div v-else-if="activeSettingsTab === 'security'" class="settings-forms">
                <!-- 安全设置内容 -->
                <SecuritySettings />
              </div>
            </div>
            
            <div v-else-if="activeTab === 'views'">
              <!-- 观点内容 -->
              <UserViews :views="userData.views" />
            </div>
            
            <div v-else-if="activeTab === 'scripts'">
              <!-- 脚本内容 -->
              <UserScripts :scripts="userData.scripts" />
            </div>
            
            <div v-else-if="activeTab === 'followers'">
              <!-- 粉丝列表 -->
              <UserFollowers :followers="userData.followers" />
            </div>
            
            <div v-else-if="activeTab === 'following'">
              <!-- 关注列表 -->
              <UserFollowing :following="userData.following" />
            </div>
          </div>
        </div>
        
        <!-- 右侧市场行情部分 -->
        <div class="market-sidebar">
          <MarketWatchlist />
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, reactive, onMounted } from 'vue';
  import UserProfileHeader from './profile/UserProfileHeader.vue';
  import UserStats from './profile/UserStats.vue';
  import ProfileTabs from './profile/ProfileTabs.vue';
  import SettingsTabs from './profile/SettingsTabs.vue';
  import PublicInfoForm from './profile/PublicInfoForm.vue';
  import PrivateInfoForm from './profile/PrivateInfoForm.vue';
  import AccountSettings from './profile/AccountSettings.vue';
  import SecuritySettings from './profile/SecuritySettings.vue';
  import UserViews from './profile/UserViews.vue';
  import UserScripts from './profile/UserScripts.vue';
  import UserFollowers from './profile/UserFollowers.vue';
  import UserFollowing from './profile/UserFollowing.vue';
  import MarketWatchlist from './market/MarketWatchlist.vue';
  
  // 接收从父组件传递的用户数据
  const props = defineProps({
    userData: {
      type: Object,
      default: () => ({
        username: 'flashdanacom',
        avatar: 'F',
        isOnline: true,
        joinDate: '2024年11月22日加入',
        email: 'user@example.com',
        stats: {
          followers: 0,
          following: 1,
          views: 0,
          scripts: 0
        },
        views: [],
        scripts: [],
        followers: [],
        following: []
      })
    }
  });
  
  // 当前激活的标签
  const activeTab = ref('settings');
  const activeSettingsTab = ref('general');
  
  // 更新用户数据
  const updateUserData = (newData: any) => {
    Object.assign(userData, newData);
  };
  
  // 更新邮箱
  const updateEmail = (newEmail: string) => {
    userData.email = newEmail;
  };
  
  // 更新密码
  const updatePassword = (newPassword: string) => {
    console.log('密码已更新:', newPassword);
    // 实际应用中这里会调用API更新密码
  };
  
  // 创建本地响应式数据，初始化为props中的数据
  const userData = reactive({...props.userData});
  
  // 当props变化时更新本地数据
  onMounted(() => {
    console.log('用户个人中心组件已加载', userData);
  });
  </script>
  
  <style lang="scss" scoped>
  .user-profile-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    min-height: 100vh;
    background-color: #131722;
    color: #d1d4dc;
  }
  
  .profile-content {
    display: flex;
    width: 100%;
    padding: 20px;
    gap: 20px;
  }
  
  .profile-main {
    flex: 1;
    max-width: 900px;
  }
  
  .market-sidebar {
    width: 320px;
    position: sticky;
    top: 20px;
    height: calc(100vh - 40px);
  }
  
  .profile-content-area {
    margin-top: 20px;
  }
  
  .settings-forms {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }
  
  @media (max-width: 1200px) {
    .profile-content {
      flex-direction: column;
    }
    
    .market-sidebar {
      width: 100%;
      position: static;
      height: auto;
    }
  }
  </style>