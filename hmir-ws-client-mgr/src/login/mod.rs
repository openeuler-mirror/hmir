use crate::client_instance;
use crate::ws_client_mgr::CLIENT_MAP;

mod login;

#[allow(dead_code)]
pub fn login(host : & str, username : &str, password : &str ) -> usize {
    let h = host.to_string();
    return client_instance!(&h).login(username, password);
}