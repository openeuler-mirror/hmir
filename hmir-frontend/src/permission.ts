import router from '@/router';
import NProgress from 'nprogress';
import 'nprogress/nprogress.css';
import { sessionStorage } from '@/utils/sessionStorage';
import { RouteRecordRaw } from 'vue-router';
import useRouterStoreHook from '@/store/modules/router';

// 进度条
NProgress.configure({ showSpinner: false });

// 白名单路由
const whiteList = ['/login', '/about', '/404'];

const useRouterStore = useRouterStoreHook()

router.beforeEach((to, from, next) => {
  NProgress.start();
  let value = sessionStorage.get('user')
  //判断用户是否登录
  if (value !== 'user') {
    //动态添加路由
    if (!useRouterStore.addRouter) {
      const accessRoutes: RouteRecordRaw[] = useRouterStore.router;
      accessRoutes.forEach((route: any) => {
        router.addRoute(route);
      });
      useRouterStore.addRouter = true;
      next({ ...to, replace: true });
      //当跳转路由包含在layout时，跳转
    } else if (useRouterStore.allRouter.includes(to.path)) {
      next();
      //如果是白名单页面直接跳转
    } else if (whiteList.includes(to.path)) {
      next();
      //只有当服务页跳转到服务详情页才有效
    } else if ((to.path.includes('serviceDetail') && from.path === '/service')
      || (from.path.includes('/') && !!to.query)) {
      next();
    } else {
      next('/404');
    }
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
