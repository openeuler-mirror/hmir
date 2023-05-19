/*
 * @Author: zhang_tianran
 * @Date: 2023-05-16 17:05:12
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-05-16 19:00:07
 * @Description: 
 */
import { defineStore } from 'pinia';
import api from '@/api';
import { store } from '../index';
import useUsersStore from '@/store/modules/user';

const userStore = useUsersStore();

export const useProcStore = defineStore('proc', {
  //存储用户数据
  state: () => {
    return {
      processAllData: [],
    };
  },
  //计算属性
  getters: {
  },
  //异步同步函数
  actions: {
    //登录
    cmd_process_info() {
      return new Promise<void>((resolve, reject) => {
        api.cmd_process_info({ host: userStore.host }).then((response: any) => {
          console.log(response)
            if (response[0] === 0) {
                let value = JSON.parse(response[1])
                let arr: any = Array.from(Object.values(value), x => x)
                this.processAllData = arr
                resolve()
            } else {
                reject(response.errmsg); 
            }
        })
      })
    },
  }
})

//在 非setup 中进行引入
export default function useProcStoreHook() {
  return useProcStore(store);
};