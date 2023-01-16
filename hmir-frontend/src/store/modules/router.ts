import { defineStore } from 'pinia';
import { store } from '../index'

//引入Layout组件
export const Layout = () => import('@/layout/index.vue');

// 第一个参数是应用程序中 store 的唯一 id
export const useRouterStore = defineStore('router', {
  //存储路由数据
  state: () => {
    return {
      router: [{
        path: '/',
        name: 'Layout',
        component: Layout,
        redirect: '/system',
        children: [
          {
            path: '/system',
            name: 'system',
            component: () => import('@/views/system/index.vue'),
            meta: { title: '系统', icon: 'Menu', disabled: false }
          },
          {
            path: '/service',
            name: 'service',
            component: () => import('@/views/service/index.vue'),
            meta: { title: '服务', icon: 'Menu', disabled: false },
          },
          {
            path: '/process',
            name: 'process',
            component: () => import('@/views/process/index.vue'),
            meta: { title: '进程', icon: 'Menu', disabled: false },
          },
          {
            path: '/serviceDetail/:serviceName(.*)',
            name: 'serviceDetail',
            component: () => import('@/views/service/components/serviceDetail/index.vue'),
            meta: { title: '服务详情', icon: 'Menu', disabled: false },
          },
          {
            path: '/console',
            name: 'console',
            component: () => import('@/views/consoleCommand/index.vue'),
            meta: { title: '控制台', icon: 'Setting', disabled: false }
          },
        ]
      }],
      //判断是否添加了路由信息
      addRouter: false,
      //所有的路由信息
      allRouter: ['/login', '/about', '/system', '/service', '/console', '/process']
    };
  },
  //计算属性
  getters: {
    userouter: state => {
      return state.router?.[0].children.filter(item => item.name !== 'console' && item.name !== 'serviceDetail')
    }
  },
  //异步同步函数
  actions: {
  }
});


//在 非setup 中进行引入
export default function useRouterStoreHook() {
  return useRouterStore(store)
}