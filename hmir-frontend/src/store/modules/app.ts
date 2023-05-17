import { defineStore } from 'pinia'
import { store } from '@/store'
import {i18n} from '@/lang'
import Cache from '@/utils/cache/index'
export const useAppStore = defineStore('app', {
  state: () => {
    return {
      locale: localStorage.getItem('lang') || 'zh_CN'
    }
  },
  actions: {
    SET_LOCALE(locale: any) { //语言切换
      this.locale = locale
      Cache.setIl8nLang(locale)
      i18n.global.locale = locale
    }
  }
})

export function useAppStoreHook() {
  return useAppStore(store)
}
