import { defineStore } from 'pinia';

export const useUsersStore = defineStore('user', {
  //存储用户数据
  state: () => {
    return {
      host: "",
      port: 0,
      username: "",
    };
  },
  //计算属性
  getters: {
  },
  //异步函数
  actions: {
  },
});
