import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import UserProfile from '@/components/UserProfile.vue';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Home',
    component: () => import('@/views/Home.vue')
  },
  {
    path: '/profile',
    name: 'profile',
    component: UserProfile,
    component: UserProfile,
    meta: {
      requiresAuth: true // 可选：如果需要登录才能访问
    }
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

// 可选：添加导航守卫，检查是否需要登录
router.beforeEach((to, from, next) => {
  const requiresAuth = to.matched.some(record => record.meta.requiresAuth);
  const isLoggedIn = !!localStorage.getItem('currentUser'); // 假设使用localStorage存储登录状态
  
  if (requiresAuth && !isLoggedIn) {
    // 如果需要登录但用户未登录，重定向到首页或登录页
    next({ name: 'Home' });
  } else {
    next();
  }
});

export default router;