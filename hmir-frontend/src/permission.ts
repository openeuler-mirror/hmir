import router from '@/router';
import { RouteRecordRaw } from 'vue-router';
import NProgress from 'nprogress';
import 'nprogress/nprogress.css';
NProgress.configure({ showSpinner: false }); // 进度条

router.beforeEach((to, from, next) => {
  NProgress.start();
});

router.afterEach(() => {
  NProgress.done();
});
