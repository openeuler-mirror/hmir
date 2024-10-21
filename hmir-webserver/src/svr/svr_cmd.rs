

use hmir_ws_client_mgr::svr::svr_mgr as clientmgr;


pub fn cmd_service_enabled(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_enabled_service(host);
}


pub fn cmd_service_disabled(host : &str) -> (usize,String) {
    return clientmgr::svr_list_disabled_service(host);
}


pub fn cmd_service_static(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_static_service(host);
}



pub fn cmd_timer_enabled(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_enabled_timer(host);
}


pub fn cmd_timer_disabled(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_disabled_timer(host);
}


pub fn cmd_timer_static(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_static_timer(host);
}




pub fn cmd_socket_enabled(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_enabled_socket(host);
}


pub fn cmd_socket_disabled(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_disabled_socket(host);
}


pub fn cmd_socket_static(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_static_socket(host);
}




pub fn cmd_target_enabled(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_enabled_target(host);
}


pub fn cmd_target_disabled(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_disabled_target(host);
}


pub fn cmd_target_static(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_static_target(host);
}



pub fn cmd_all_slice(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_all_slice(host);
}