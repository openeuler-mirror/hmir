use crate::client_instance;
use crate::ws_client_mgr::CLIENT_MAP;
use crate::ws_client_mgr::unregister_client;


pub fn proc_process_info(host : & str) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).proc_process_info();
}

