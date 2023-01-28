import { defineStore } from 'pinia';
import { cmd_get_ceph_status } from '@/api/ceph/index';
import { store } from '../index';
import useUsersStore from '@/store/modules/user';

const userStore = useUsersStore();
// 第一个参数是应用程序中 store 的唯一 id
export const useCephStore = defineStore('ceph', {
  //存储用户数据
  state: () => {
    return {
      cmdCephStatus: '',
    };
  },
  //计算属性
  getters: {
  },
  //异步同步函数
  actions: {
    //登录
    cmd_get_ceph_status() {
      return new Promise<void>((resolve, reject) => {
        cmd_get_ceph_status({ host: userStore.host })
          .then((response: any) => {
            if (response.code === 0) {
              this.cmdCephStatus = JSON.parse(response.result);
              console.log(this.cmdCephStatus);
              resolve()
            } else {
              reject(response.errmsg);
            }
          })
          .catch(error => {
            reject(error);
          });
      })
    },
  }
});

//在 非setup 中进行引入
export default function useCephStoreHook() {
  return useCephStore(store)
};
