import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue')
  },
  {
    path: '/profile',
    name: 'UserProfile',
    component: () => import('../components/UserProfile.vue')
  }
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