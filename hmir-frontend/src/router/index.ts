import { createRouter, createWebHashHistory } from 'vue-router';

export const Layout = () => import('@/layout/index.vue');

// 静态路由
export const constantRoutes = [
  {
    path: '/login',
    component: () => import('@/views/login/index.vue'),
  },
  // {
  //   path: '/404',
  //   component: () => import('@/views/error-page/404.vue'),
  // },
  {
    path: '/',
    component: Layout,
    redirect: '/home',
    children: [
      {
        path: 'home',
        component: () => import('@/views/home/index.vue'),
        name: 'home',
      },
      {
        path: 'console',
        component: () => import('@/views/consoleCommand/index.vue'),
      }
    ]
  }
];

// 创建路由
const router = createRouter({
  history: createWebHashHistory(),
  routes: constantRoutes,
  // 刷新时，滚动条位置还原
  scrollBehavior: () => ({ left: 0, top: 0 })
});

export default router;