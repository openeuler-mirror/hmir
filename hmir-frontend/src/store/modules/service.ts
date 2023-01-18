import { defineStore } from 'pinia';
import ElMessage from '@/utils/message';
import {
  cmd_service_enabled, cmd_service_disabled, cmd_service_static,
  cmd_timer_enabled, cmd_timer_disabled, cmd_timer_static
} from '@/api/index';
import { store } from '../index';
import useUsersStore from '@/store/modules/user';

const userStore = useUsersStore();

// 第一个参数是应用程序中 store 的唯一 id
export const cmdServiceStore = defineStore('servive', {
  //存储servive数据
  state: () => {
    return {
      cmdServiceClickDetail: {
        serviceActive: 'systemService',
        serviceCollapse: '',
        serviceTable: '',
      },
      serviceDetail: {} as any,
      serviceAll: {
        //系统服务
        cmdServiceEnabled: [],
        cmdServiceDisabled: [],
        cmdServiceStatic: [],
        //计时器
        cmdTimerEnabled: [],
        cmdTimerDisabled: [],
        cmdTimerStatic: [],
      }
    };
  },
  //计算属性
  getters: {
  },
  //异步同步函数
  actions: {
    //请求所有数据
    cmd_service_all() {
      return new Promise<void>((resolve, reject) => {
        let timeout = 200
        if (this.serviceAll.cmdServiceEnabled.length !== 0) { timeout = 300 }
        setTimeout(() => {
          this.cmd_service_enabled();
          this.cmd_service_disabled();
          this.cmd_service_static();
          this.cmd_timer_enabled();
          this.cmd_timer_disabled();
          this.cmd_timer_static();
          resolve()
        }, timeout);
      })
    },
    //获取其中一条数据
    service_detail(name: string | string[]) {
      let item: any, value: any
      for (item in this.serviceAll) {
        for (value of this.serviceAll[item]) {
          if (value.name === name) {
            this.serviceDetail = value;
            break;
          }
        }
      }
    },
    //判断当前数据是否在所有数据里面拥有，有则可以点击
    is_service_disabled(name: string | string[]) {
      let isDisabled = true, item: any, value: any
      for (item in this.serviceAll) {
        for (value of this.serviceAll[item]) {
          if (value.name === name) {
            isDisabled = false;
            break;
          }
        }
      }
      return isDisabled;
    },
    //系统服务启用
    cmd_service_enabled() {
      return new Promise<void>((resolve, reject) => {
        cmd_service_enabled({ host: userStore.host }).then((res: any) => {
          if (res[0] === 0) {
            let value: any = JSON.parse(res[1]);
            let arr: any = Array.from(Object.values(value), x => x);
            this.serviceAll.cmdServiceEnabled = arr;
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
          if (res[0] === 0) {
            let value: any = JSON.parse(res[1]);
            let arr: any = Array.from(Object.values(value), x => x);
            this.serviceAll.cmdServiceDisabled = arr;
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
          if (res[0] === 0) {
            let value: any = JSON.parse(res[1]);
            let arr: any = Array.from(Object.values(value), x => x);
            this.serviceAll.cmdServiceStatic = arr;
            resolve()
          } else {
            reject('获取系统服务静态信息失败');
          }
        })
      })
    },

    //计时器的启动
    cmd_timer_enabled() {
      return new Promise<void>((resolve, reject) => {
        cmd_timer_enabled({ host: userStore.host }).then((res: any) => {
          if (res[0] === 0) {
            let value: any = JSON.parse(res[1]);
            let arr: any = Array.from(Object.values(value), x => x);
            this.serviceAll.cmdTimerEnabled = arr;
            resolve()
          } else {
            reject('获取计时器启用信息失败');
          }
        })
      })
    },

    //计时器禁用
    cmd_timer_disabled() {
      return new Promise<void>((resolve, reject) => {
        cmd_timer_disabled({ host: userStore.host }).then((res: any) => {
          if (res[0] === 0) {
            let value: any = JSON.parse(res[1]);
            let arr: any = Array.from(Object.values(value), x => x);
            this.serviceAll.cmdTimerDisabled = arr;
            resolve()
          } else {
            reject('获取计时器禁用信息失败');
          }
        })
      })
    },

    //计时器静态
    cmd_timer_static() {
      return new Promise<void>((resolve, reject) => {
        cmd_timer_static({ host: userStore.host }).then((res: any) => {
          if (res[0] === 0) {
            let value: any = JSON.parse(res[1]);
            let arr: any = Array.from(Object.values(value), x => x);
            this.serviceAll.cmdTimerStatic = arr;
            resolve()
          } else {
            reject('获取计时器静态信息失败');
          }
        })
      })
    },
  }
});

//在 非setup 中进行引入
export default function cmdServiceStoreHook() {
  return cmdServiceStore(store)
};
