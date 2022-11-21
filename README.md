# HMIR

#### 介绍
Host management in rust

#### 软件架构
软件架构说明

- src/main.rs : 主框架,实现ws的RPC框架
- src/xxxx.rs : 接口模块，每个模块暴露 register_method，完成模块的rpc接口注册
- hmir-xxx: 这些Crate和主机底层交互，由于当前rust处于发展阶段，很多Rust Crate还不够成熟，
  或者缺少对应的库，因此这里会使用bindgen技术，对C库进行bind和封装，提供给src/xxxx.rs调用。



#### 安装教程

1.  make install

#### 使用说明

1. 默认安装在 ~/.cargo/bin/目录下
2. 运行

```
sudo ./hmir --host 127.0.0.1 --port 8080
```
3. 监听地址: 127.0.0.1:8080地址
4. 可通过 ws://127.0.0.1:8080 发送websocket请求

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 特技

1.  使用 Readme\_XXX.md 来支持不同的语言，例如 Readme\_en.md, Readme\_zh.md
2.  Gitee 官方博客 [blog.gitee.com](https://blog.gitee.com)
3.  你可以 [https://gitee.com/explore](https://gitee.com/explore) 这个地址来了解 Gitee 上的优秀开源项目
4.  [GVP](https://gitee.com/gvp) 全称是 Gitee 最有价值开源项目，是综合评定出的优秀开源项目
5.  Gitee 官方提供的使用手册 [https://gitee.com/help](https://gitee.com/help)
6.  Gitee 封面人物是一档用来展示 Gitee 会员风采的栏目 [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)
