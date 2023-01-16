import { defineStore } from 'pinia';
import ElMessage from '@/utils/message';
import { cmd_service_enabled, cmd_service_disabled, cmd_service_static } from '@/api/index';
import { store } from '../index';
import useUsersStore from '@/store/modules/user';

const userStore = useUsersStore();

// 第一个参数是应用程序中 store 的唯一 id
export const cmdServiceStore = defineStore('servive', {
  //存储servive数据
  state: () => {
    return {
      cmdServiceClickDetail: {},
      cmdServiceEnabled: [],
      cmdServiceDisabled: [],
      cmdServiceStatic: [],
    };
  },
  //计算属性
  getters: {
  },
  //异步同步函数
  actions: {
    //系统服务启用
    cmd_service_enabled() {
      return new Promise<void>((resolve, reject) => {
        cmd_service_enabled({ host: userStore.host }).then((res: any) => {
          console.log(res);
          console.log(res[0]);
          if (res[0] === 0) {
            let value: any = JSON.parse(res[1]);
            let arr: any = Array.from(Object.values(value), x => x);
            this.cmdServiceEnabled = arr;
            resolve()
          } else {
            reject('获取系统服务启用信息失败');
          }
        })
      })
    },

    //系统服务禁用
    cmd_service_disabled() {
      return new Promise<void>((resolve, reject) => {
        cmd_service_disabled({ host: userStore.host }).then((res: any) => {
          console.log(res);
          console.log(res[0]);
          if (res[0] === 0) {
            let value: any = JSON.parse(res[1]);
            let arr: any = Array.from(Object.values(value), x => x);
            this.cmdServiceDisabled = arr;
            resolve()
          } else {
            reject('获取系统服务禁用信息失败');
          }
        })
      })
    },

    //系统服务静态
    cmd_service_static() {
      return new Promise<void>((resolve, reject) => {
        cmd_service_static({ host: userStore.host }).then((res: any) => {
          console.log(res);
          console.log(res[0]);
          if (res[0] === 0) {
            let value: any = JSON.parse(res[1]);
            let arr: any = Array.from(Object.values(value), x => x);
            this.cmdServiceStatic = arr;
            resolve()
          } else {
            reject('获取系统服务静态信息失败');
          }
        })
      })
    }
  }
});

//在 非setup 中进行引入
export default function cmdServiceStoreHook() {
  return cmdServiceStore(store)
};
