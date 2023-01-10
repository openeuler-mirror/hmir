import router from '@/router';
import NProgress from 'nprogress';
import 'nprogress/nprogress.css';
import { sessionStorage } from '@/utils/sessionStorage'
NProgress.configure({ showSpinner: false }); // 进度条

// 白名单路由
const whiteList = ['/login', '/about'];

router.beforeEach((to, from, next) => {
  NProgress.start();
  let value = sessionStorage.get('user')
  //判断用户是否登录
  if (value !== 'user') {
    next();
  } else {
    // 未登录可以访问白名单页面
    if (whiteList.includes(to.path)) {
      next();
    } else {
      next(`/login`);
      NProgress.done();
    }
  }
});

router.afterEach(() => {
  NProgress.done();
});
