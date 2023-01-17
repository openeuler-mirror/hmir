use crate::client_instance;
use crate::ws_client_mgr::CLIENT_MAP;
use crate::ws_client_mgr::unregister_client;










pub fn svr_list_disabled_service(host : & str) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).svr_list_disabled_service();
}

pub fn svr_list_enabled_service(host : & str) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).svr_list_enabled_service();
}

pub fn svr_list_static_service(host : & str) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).svr_list_static_service();
}


pub fn svr_list_disabled_timer(host : & str) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).svr_list_disabled_timer();
}

pub fn svr_list_enabled_timer(host : & str) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).svr_list_enabled_timer();
}

pub fn svr_list_static_timer(host : & str) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).svr_list_static_timer();
}