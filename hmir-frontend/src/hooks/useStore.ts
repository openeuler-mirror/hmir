/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-29 17:23:09
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-05 11:25:42
 * @FilePath: /hmir-frontend/src/hooks/useStore.ts
 * @Description:
 */
import { storeToRefs } from 'pinia'
import useAppStoreHook from '@/store/modules/app'
import useHostStoreHook from '@/store/modules/cluster/host'
import useHeightStoreHook from '@/store/modules/appMainHeight'
import useCephStoreHook from '@/store/modules/ceph'
import useProcStoreHook from '@/store/modules/proc'
import useRouterStoreHook from '@/store/modules/router'
import cmdServiceStoreHook from '@/store/modules/service'
import useUsersStoreHook from '@/store/modules/user'

const storeTypeMap = new Map<string, Function>([
    ['app', useAppStoreHook],
    ['host', useHostStoreHook],
    ['mainHeight', useHeightStoreHook],
    ['ceph', useCephStoreHook],
    ['proc', useProcStoreHook],
    ['router', useRouterStoreHook],
    ['service', cmdServiceStoreHook],
    ['user', useUsersStoreHook]
])

/**
 * @description: 初始化Store仓库使用
 * @param {String} type 你要使用的仓库
 * @return {Object}
 */
export default function initStoreUse(type: 'app' | 'host' | 'mainHeight' | 'ceph' | 'proc' | 'router' | 'service' | 'user'): any {
    const store = storeTypeMap.get(type)?.() as any

    return {
        store,
        ...storeToRefs(store)
    }
}