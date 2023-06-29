import { defineStore } from 'pinia';
import { store } from '../index'
import { i18n } from '@/lang/index'

const { t } = i18n.global
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
            meta: { title: t('system'), icon: 'Menu', disabled: false }
          },
          {
            path: '/clusterHealth',
            name: 'ceph',
            redirect: '/clusterHealth/dashBoard',
            // component: () => import('@/views/ceph/index.vue'),
            meta: { title: 'Ceph', icon: 'Menu', disabled: false },
            children: [
              {
                path: '/clusterHealth/dashBoard',
                name: 'dashBoard',
                component: () => import('@/views/ceph/dashBoard/index.vue'),
                meta: { title: t('instrumentPanel'), icon: 'Menu', disabled: false }
              },
              {
                path: '/clusterHealth/cluster',
                name: 'cluster',
                meta: { title: t('cluster'), icon: 'Menu', disabled: false },
                redirect: '/cluster/hosts',
                children: [
                  {
                    path: '/clusterHealth/cluster/hosts',
                    name: 'hosts',
                    component: () => import('@/views/cluster/hosts/index.vue'),
                    meta: { title: t('hosts'), icon: 'Menu', disabled: false }
                  }
                ]
              },
            ]
          },
          {
            path: '/service',
            name: 'service',
            // redirect: '/service/systemService',
            component: () => import('@/views/service/index.vue'),
            meta: { title: t('service'), icon: 'Menu', disabled: false },
            // children: [
            //   {
            //     path: '/service/serviceTarget',
            //     name: 'serviceTarget',
            //     component: () => import('@/views/service/components/serviceTarget/index.vue'),
            //     meta: { title: '目标', icon: 'Menu', disabled: false },
            //   },
            //   {
            //     path: '/service/systemService',
            //     name: 'systemService',
            //     component: () => import('@/views/service/components/systemService/index.vue'),
            //     meta: { title: '系统服务', icon: 'Menu', disabled: false },
            //   },
            //   {
            //     path: '/service/serviceSocket',
            //     name: 'serviceSocket',
            //     component: () => import('@/views/service/components/serviceSocket/index.vue'),
            //     meta: { title: '套接字', icon: 'Menu', disabled: false },
            //   },
            //   {
            //     path: '/service/serviceTimer',
            //     name: 'serviceTimer',
            //     component: () => import('@/views/service/components/serviceTimer/index.vue'),
            //     meta: { title: '计时器', icon: 'Menu', disabled: false },
            //   },
            //   {
            //     path: '/service/servicePath',
            //     name: 'servicePath',
            //     component: () => import('@/views/service/components/servicePath/index.vue'),
            //     meta: { title: '路径', icon: 'Menu', disabled: false },
            //   }
            // ]
          },
          {
            path: '/process',
            name: 'process',
            component: () => import('@/views/process/index.vue'),
            meta: { title:t('process'), icon: 'Menu', disabled: false },
          },
          {
            path: '/virtual',
            name: 'virtual',
            component: () => import('@/views/virt/index.vue'),
            meta: { title: t('vm'), icon: 'Menu', disabled: false },
          },
          {
            path: '/serviceDetail/:serviceName(.*)',
            name: 'serviceDetail',
            component: () => import('@/views/service/components/serviceDetail/index.vue'),
            meta: { title: t('serviceDetails'), icon: 'Menu', disabled: false },
          },
          {
            path: '/console',
            name: 'console',
            component: () => import('@/views/consoleCommand/index.vue'),
            meta: { title: t('console'), icon: 'Setting', disabled: false }
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