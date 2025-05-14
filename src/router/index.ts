import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import UserProfile from '@/components/UserProfile.vue';
import MarketWatchlist from '@/views/marketWatch/MarketWatchlist.vue';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue')
  },
  {
    path: '/profile',
    name: 'profile',
    component: UserProfile,
    meta: {
      requiresAuth: true // 可选：如果需要登录才能访问
    }
  },
  {
    path: '/market-watchlist',
    name: 'market-watchlist',
    component: MarketWatchlist,
    meta: {
      requiresAuth: true // 可选：如果需要登录才能访问
    }
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

// 添加导航日志，帮助调试
router.beforeEach((to, from, next) => {
  console.log(`导航从 ${from.path} 到 ${to.path}`);
  next();
});

export default router;