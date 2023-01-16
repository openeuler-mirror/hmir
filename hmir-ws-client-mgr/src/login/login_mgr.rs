use crate::client_instance;
use crate::ws_client_mgr::CLIENT_MAP;
use crate::ws_client_mgr::unregister_client;


#[allow(dead_code)]
pub fn ssh_login(host : & str, username : &str,password : &str) -> usize {
    let h = host.to_string();
    return client_instance!(&h).ssh_login(username, password);
}

#[allow(dead_code)]
pub fn login(host : & str, username : &str, password : &str ) -> usize {
    let h = host.to_string();
    return client_instance!(&h).login(username, password);
}

/// 注销系统
#[allow(dead_code)]
pub fn logout(host : & str) -> bool
{
    return unregister_client(host);
}