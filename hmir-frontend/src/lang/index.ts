import type { App } from 'vue';
import { createI18n } from 'vue-i18n'
import en from './en'
import zh from './zh'

const messages = {
    'zh_CN': zh,
    'en_US': en
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