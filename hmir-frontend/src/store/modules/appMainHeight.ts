/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-29 16:17:53
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-05 11:26:43
 * @FilePath: /hmir-frontend/src/store/modules/appMainHeight.ts
 * @Description: 
 */
import { defineStore } from 'pinia'
import { store } from '@/store'

export const useHeightStore = defineStore('height', {
  state: () => {
    return {
      appMainHeight: 650
    }
  },
  actions: {
    UPDATE_HEIGHT(height: number) { //更新高度
      this.appMainHeight = height
    }
  }
})

export default function useHeightStoreHook() {
  return useHeightStore(store)
}
