import { createRouter, createWebHashHistory } from 'vue-router';

export const Layout = () => import('../layout/index.vue');

// 静态路由
export const constantRoutes = [
  // {
  //   path: '/redirect',
  //   component: Layout,
  //   meta: { hidden: true },
  //   children: [
  //     {
  //       path: '/redirect/:path(.*)',
  //       component: () => import('./views/redirect/index.vue')
  //     }
  //   ]
  // },
  // {
  //   path: '/login',
  //   component: () => import('@/views/login/index.vue'),
  //   meta: { hidden: true }
  // },
  // {
  //   path: '/404',
  //   component: () => import('@/views/error-page/404.vue'),
  //   meta: { hidden: true }
  // },

  {
    path: '/',
    component: Layout,
    redirect: '/home',
    children: [
      {
        path: 'home',
        component: () => import('../views/home/index.vue'),
        name: 'home',
      },
      {
        path: 'console',
        component: () => import('../views/consoleCommand/index.vue'),
        meta: { hidden: true }
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
