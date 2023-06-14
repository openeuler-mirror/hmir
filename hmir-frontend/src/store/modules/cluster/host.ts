/*
 * @Author: zhang_tianran
 * @Date: 2023-06-14 10:48:59
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-14 13:19:30
 * @Description: 
 */
import { defineStore } from 'pinia';
import { store } from '@/store/index';

export const hostsProcStore = defineStore('hosts', {
  //存储用户数据
  state: () => {
    return {
      defaultBreadcrumbTitle: ['cluster'],
    };
  },
  //计算属性
  getters: {
  },
  //异步同步函数
  actions: {
    //获取默认title
    get_defaultTitle(title: Array<string>) {
      const defaultTitle = [...this.defaultBreadcrumbTitle, ...title]
      return defaultTitle
   }
  }
})

//在 非setup 中进行引入
export default function useProcStoreHook() {
  return hostsProcStore(store);
};
