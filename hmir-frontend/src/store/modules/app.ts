/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 09:38:36
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-29 14:41:14
 * @FilePath: /hmir-frontend/src/store/modules/app.ts
 * @Description: 
 */
import { defineStore } from 'pinia'
import { store } from '@/store'
import {i18n} from '@/lang'
import Cache from '@/utils/cache/index'
export const useAppStore = defineStore('app', {
  state: () => {
    return {
      locale: Cache.getIl8nLang() || 'zh_CN'
    }
  },
  actions: {
    SET_LOCALE(locale: any) { //语言切换
      this.locale = locale
      Cache.setIl8nLang(locale)
      i18n.global.locale.value = locale
    }
  }
})

export function useAppStoreHook() {
  return useAppStore(store)
}
