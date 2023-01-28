import { defineStore } from 'pinia';
import {
  cmd_service_enabled, cmd_service_disabled, cmd_service_static,
  cmd_timer_enabled, cmd_timer_disabled, cmd_timer_static,
  cmd_socket_enabled, cmd_socket_disabled, cmd_socket_static,
  cmd_target_enabled, cmd_target_disabled, cmd_target_static,
  cmd_all_slice,
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
      //保存详情页的数据
      serviceDetail: {} as any,
      //用来保存所有处理后的数据
      serviceAll: {
        cmdAllSlice: [],
        //系统服务
        cmdServiceEnabled: [],
        cmdServiceDisabled: [],
        cmdServiceStatic: [],
        //计时器
        cmdTimerEnabled: [],
        cmdTimerDisabled: [],
        cmdTimerStatic: [],
        //套接字
        cmdSocketEnabled: [],
        cmdSocketDisabled: [],
        cmdSocketStatic: [],
        //目标
        cmdTargetEnabled: [],
        cmdTargetDisabled: [],
        cmdTargetStatic: [],
      },
      //用来保存所有请求来的JSON格式化后的原始数据，通过对象保存方便进行索引
      serviceAllData: {},
      //用来保存所有的api接口
      serviceAllApi: {
        'cmdAllSlice': {
          apiFunction: cmd_all_slice
        },
        'cmdServiceEnabled': {
          apiFunction: cmd_service_enabled
        },
        'cmdServiceDisabled': {
          apiFunction: cmd_service_disabled
        },
        'cmdServiceStatic': {
          apiFunction: cmd_service_static
        },
        'cmdTimerEnabled': {
          apiFunction: cmd_timer_enabled
        },
        'cmdTimerDisabled': {
          apiFunction: cmd_timer_disabled
        },
        'cmdTimerStatic': {
          apiFunction: cmd_timer_static
        },
        'cmdSocketEnabled': {
          apiFunction: cmd_socket_enabled
        },
        'cmdSocketDisabled': {
          apiFunction: cmd_socket_disabled
        },
        'cmdSocketStatic': {
          apiFunction: cmd_socket_static
        },
        'cmdTargetEnabled': {
          apiFunction: cmd_target_enabled
        },
        'cmdTargetDisabled': {
          apiFunction: cmd_target_disabled
        },
        'cmdTargetStatic': {
          apiFunction: cmd_target_static
        },
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
        //清空之前保存的数据
        this.serviceAllData = {};
        let timeout = 200;
        //判断之前本地是否拥有数据，
        if (this.serviceAll.cmdServiceEnabled.length !== 0) { timeout = 300 }
        setTimeout(() => {
          for (let item in this.serviceAllApi) {
            this.cmd_service_request(item)
          }
          resolve()
        }, timeout);
      })
    },

    //获取其中一条数据
    service_detail(name: string | string[]) {
      this.serviceDetail = this.serviceAllData[name.toString()] || {}
    },

    //判断当前数据是否在所有数据里面拥有，有则可以点击
    is_service_disabled(name: string | string[]) {
      return !this.serviceAllData[name.toString()];
    },

    //请求接口函数
    cmd_service_request(api: string) {
      return new Promise<void>((resolve, reject) => {
        this.serviceAllApi[api].apiFunction({ host: userStore.host })
          .then((res: any) => {
            if (res[0] === 0) {
              let value: any = JSON.parse(res[1]);
              Object.assign(this.serviceAllData, value)
              let arr: any = Array.from(Object.values(value), x => x);
              this.serviceAll[api] = arr;
              resolve()
            } else {
              reject(`获取${api}信息失败`);
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
export default function cmdServiceStoreHook() {
  return cmdServiceStore(store)
};
