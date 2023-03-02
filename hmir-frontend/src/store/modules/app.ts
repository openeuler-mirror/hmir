import { defineStore } from 'pinia'
import { store } from '@/store'
import {i18n} from '@/lang'

export const useAppStore = defineStore('app', {
  state: () => {
    return {
      locale: localStorage.getItem('lang') || 'zh_CN'
    }
  },
  actions: {
    SET_LOCALE(locale: any) { //语言切换
      this.locale = locale
      localStorage.setItem('lang', locale)
      i18n.global.locale = locale
    }
  }
})

export function useAppStoreHook() {
  return useAppStore(store)
}
