import type { App } from 'vue';
import { createI18n } from 'vue-i18n'
import enLocale from 'element-plus/es/locale/lang/en'
import zhLocale from 'element-plus/es/locale/lang/zh-cn'

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
    locale: localStorage.getItem('lang') || 'zh_CN',
    messages
}

export const i18n = createI18n(localData)

export function setupI18n(app: App<Element>) {
    app.use(i18n);
}