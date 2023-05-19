/*
 * @Author: zhang_tianran
 * @Date: 2023-05-16 17:05:12
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-05-17 15:59:11
 * @Description: 
 */
import type { App } from 'vue';
import { createI18n } from 'vue-i18n'
import enLocale from 'element-plus/es/locale/lang/en'
import zhLocale from 'element-plus/es/locale/lang/zh-cn'
import Cache from '@/utils/cache/index'
import en from './en'
import zh from './zh'

// const messages = {
//     'zh_CN': zh,
//     'en_US': en
// }
const messages = {
    'en_US': {
        ...en,
        ...enLocale
    },
    'zh_CN': {
       ...zh,
       ...zhLocale
    }
}

const localData = {
    globalInjextion: true,
    legacy: false,
    locale: Cache.getIl8nLang() || 'zh_CN',
    messages
}

export const i18n = createI18n(localData)

export function setupI18n(app: App<Element>) {
    app.use(i18n);
}