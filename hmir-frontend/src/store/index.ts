import type { App } from 'vue';
import { createPinia } from 'pinia';

const store = createPinia();

// 全局挂载store
//Pinia 是 Vue.js 的轻量级状态管理库，Vuex 的替代方案。
export function setupStore(app: App<Element>) {
  app.use(store);
}

export { store };
