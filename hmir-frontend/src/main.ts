/*
 * @Author: duanwujie88 dev17001@linx-info.com
 * @Date: 2023-02-02 14:20:40
 * @LastEditors: duanwujie88 dev17001@linx-info.com
 * @LastEditTime: 2023-02-02 17:14:57
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
