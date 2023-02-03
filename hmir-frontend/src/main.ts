import { createApp } from "vue";
import ElementPlus from 'element-plus';
import router from "./router";
import App from "./App.vue";
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import { deBounce } from './directive/index';
import { setupStore } from '@/store';
import { s3Layer } from 'vue3-layer';
import 'virtual:svg-icons-register';
import '@/permission';
import 'vue3-layer/dist/s3Layer.css';
import 'normalize.css/normalize.css';
import 'element-plus/dist/index.css';
import "./style.css";
import 'animate.css';
import { setupI18n } from "./lang";

const app = createApp(App)
app.directive('deBounce', deBounce);
//全局挂载
setupStore(app);
setupI18n(app);
app.use(ElementPlus).use(router)
app.component('s3-layer', s3Layer);
app.mount('#app')
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}
