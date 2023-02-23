use crate::client_instance;
use crate::mgr_fn_define;
use crate::ws_client_mgr::CLIENT_MAP;
use crate::ws_client_mgr::unregister_client;

/// 启动终端
mgr_fn_define!(ttyd_start);

/// 停止终端
mgr_fn_define!(ttyd_stop);


