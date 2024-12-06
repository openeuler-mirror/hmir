/*
 * @Author: zhang_tianran
 * @Date: 2023-05-16 17:05:12
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 14:43:43
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
    messages,
    silentTranslationWarn: true,
    fallbackWarn: false,
    missingWarn: false
}

export const i18n = createI18n(localData)

export function setupI18n(app: App<Element>) {
    app.use(i18n);
}