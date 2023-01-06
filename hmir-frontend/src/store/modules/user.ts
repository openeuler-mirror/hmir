import { defineStore } from 'pinia';

// 第一个参数是应用程序中 store 的唯一 id
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
  //异步同步函数
  actions: {
    cmdLogin(loginData) {
      return new Promise<void>((resolve, reject) => {
        invoke("cmd_login", { ...loginData })
          .then(response => {
            console.log(response);
            if (response) {
              ElMessage.success('登录成功')
              resolve();
            } else {
              ElMessage({
                message: '登录失败，请重试',
                type: 'error',
                customClass: 'login-message-error',
              })
              reject(response);
            }
          })
          .catch(error => {
            reject(error);
          });
      })
    }
  }
});
