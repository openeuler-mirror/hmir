/*
 * @Author: duanwujie88 dev17001@linx-info.com
 * @Date: 2023-02-02 14:20:40
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-05 15:41:30
 * @FilePath: /hmir-frontend/src/main.ts
 * @Description: hmir-front
 */
/*
 *                        _oo0oo_
 *                       o8888888o
 *                       88" . "88
 *                       (| -_- |)
 *                       0\  =  /0
 *                     ___/`---'\___
 *                   .' \\|     |// '.
 *                  / \\|||  :  |||// \
 *                 / _||||| -:- |||||- \
 *                |   | \\\  - /// |   |
 *                | \_|  ''\---/''  |_/ |
 *                \  .-\__  '-'  ___/-. /
 *              ___'. .'  /--.--\  `. .'___
 *           ."" '<  `.___\_<|>_/___.' >' "".
 *          | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *          \  \ `_.   \_ __\ /__ _/   .-` /  /
 *      =====`-.____`.___ \_____/___.-`___.-'=====
 *                        `=---='
 * 
 * 
 *      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * 
 *            佛祖保佑     永不宕机     永无BUG
 */

import { createApp } from "vue";
import ElementPlus, { ConfigProviderProps } from 'element-plus';
import router from "./router";
import App from "./App.vue";
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import { deBounce } from './directive/index';
import { setupStore } from '@/store';
import 'virtual:svg-icons-register';
import '@/permission';
import 'normalize.css/normalize.css';
import 'element-plus/dist/index.css';
import "./styles/style.css";
import "./styles/index.scss";
import 'animate.css';
import echarts from "./utils/echarts";
import { setupI18n, i18n } from "./lang";

const { t } = i18n.global
const app = createApp(App)
app.directive('deBounce', deBounce);
//全局挂载
setupStore(app);
setupI18n(app);
app.use(ElementPlus, {
  il8n: (plugin: any, options?: Partial<ConfigProviderProps> | undefined) => t(plugin, options)
}).use(router)

app.config.globalProperties.$echarts = echarts

Object.defineProperty(window, '$vueApp', {
  value: 'app'
})
app.mount('#app')
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}
