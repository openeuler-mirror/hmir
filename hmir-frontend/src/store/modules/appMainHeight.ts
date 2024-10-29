/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-29 16:17:53
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-29 16:21:08
 * @FilePath: /hmir-frontend/src/store/modules/appMainHeight.ts
 * @Description: 
 */
import { defineStore } from 'pinia'
import { store } from '@/store'
import {i18n} from '@/lang'
import Cache from '@/utils/cache/index'
export const useHeightStore = defineStore('height', {
  state: () => {
    return {
        appMainHeight: 650
    }
  },
  actions: {
    UPDATE_HEIGHT(height: number) { //语言切换
      this.appMainHeight = height
    }
  }
})

export function useHeightStoreHook() {
  return useHeightStore(store)
}
