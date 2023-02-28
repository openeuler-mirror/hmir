use crate::client_instance;
use crate::mgr_fn_define;
use crate::ws_client_mgr::CLIENT_MAP;


mgr_fn_define!(svr_list_disabled_service);
mgr_fn_define!(svr_list_enabled_service);
mgr_fn_define!(svr_list_static_service);
mgr_fn_define!(svr_list_disabled_timer);
mgr_fn_define!(svr_list_enabled_timer);
mgr_fn_define!(svr_list_static_timer);
mgr_fn_define!(svr_list_disabled_socket);
mgr_fn_define!(svr_list_enabled_socket);
mgr_fn_define!(svr_list_static_socket);
mgr_fn_define!(svr_list_disabled_target);
mgr_fn_define!(svr_list_enabled_target);
mgr_fn_define!(svr_list_static_target);
mgr_fn_define!(svr_list_all_slice);