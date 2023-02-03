
import { defineStore } from 'pinia';
import api from '@/api';
import { store } from '../index';

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
          
      })
    },
  }
})