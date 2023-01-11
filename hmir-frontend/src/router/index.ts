import { createRouter, createWebHashHistory } from 'vue-router';
import useRouterStoreHook from '@/store/modules/router'

// 静态路由
export const constantRoutes = [
  {
    path: '/login',
    name: 'login',
    component: () => import('@/views/login/index.vue'),
  },
  {
    path: '/404',
    name: '404',
    component: () => import('@/views/errorPage/404.vue'),
  },
  {
    path: '/about',
    name: 'about',
    component: () => import('@/views/windowHeader/about/index.vue'),
  }
];

// 创建路由
const router = createRouter({
  history: createWebHashHistory(),
  routes: constantRoutes,
  // 刷新时，滚动条位置还原
  scrollBehavior: () => ({ left: 0, top: 0 })
});

// 重置路由
export function resetRouter() {
  const permissionStore = useRouterStoreHook();
  permissionStore.addRouter = false;
  permissionStore.router.forEach(route => {
    const name = route.name;
    if (name && router.hasRoute(name)) {
      router.removeRoute(name);
    }
  });
}

export default router;
