## 新增tauri接口

### hmir-ws-client

在该包中,新增对应模块的ws-client rpc接口调用,作为到server端的接口层

### hmir-ws-client-mgr

在该包中,新增ws-client rpc的接口封装,为tarui注册接口的调用

### src-tauri

在该包中,新增tauri-common接口,调用client-mgr

main.rs中新增tauri-common接口的注册