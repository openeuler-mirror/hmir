use crate::client_instance;
use crate::ws_client_mgr::CLIENT_MAP;
use crate::ws_client_mgr::unregister_client;
/// 启动终端
#[allow(dead_code)]
pub fn ttyd_start(host : & str) -> (usize,String) {
    let h = host.to_string();
    return client_instance!(&h).ttyd_start();
}

/// 停止终端
#[allow(dead_code)]
pub fn ttyd_stop(host : & str) -> (usize,String) {
    let h = host.to_string();
    return client_instance!(&h).ttyd_stop();
}


