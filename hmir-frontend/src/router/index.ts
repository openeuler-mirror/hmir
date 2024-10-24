/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 09:38:36
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-23 17:22:02
 * @FilePath: /hmir-frontend/src/router/index.ts
 * @Description:
 */
import { createRouter, createWebHashHistory } from 'vue-router';
import useRouterStoreHook from '@/store/modules/router';
import cmdServiceStoreHook from '@/store/modules/service';

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
  },
  {
    path: '/linxInfo',
    name: 'linxInfo',
    component: () => import('@/views/linxInfo/index.vue'),
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
  //重置pinia所有信息
  const permissionStore = useRouterStoreHook();
  const cmdServiceStore = cmdServiceStoreHook();
  cmdServiceStore.$reset();
  permissionStore.$reset();
  permissionStore.router.forEach(route => {
    const name = route.name;
    if (name && router.hasRoute(name)) {
      router.removeRoute(name);
    }
  });
}

export default router;
