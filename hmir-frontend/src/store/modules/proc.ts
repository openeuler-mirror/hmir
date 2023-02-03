
import { defineStore } from 'pinia';
import api from '@/api';
import { store } from '../index';
import useUsersStore from '@/store/modules/user';

const userStore = useUsersStore();

export const useProcStore = defineStore('proc', {
  //存储用户数据
  state: () => {
    return {
      processAllData: {},
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
            if (response.code === 0) {
                this.processAllData = JSON.parse(response.result);
                resolve()
            } else {
                reject(response.errmsg); 
            }
        })

      })
    },
  }
})