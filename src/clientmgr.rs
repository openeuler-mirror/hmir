

use crate::wsclient;
use std::sync::{Arc,Mutex};
use hmir_hash::HashWrap;
use crate::wsclient::RequestClient;
use lazy_static::lazy_static;
use std::cell::RefCell;


struct ClientInstance{
    host : String,
    port : i32,
    token : String,
    pub client : RefCell<RequestClient>
}

type ClientType = Arc<Mutex<HashWrap<String,ClientInstance>>>;

lazy_static! {
    static ref CLIENT_MAP: ClientType = {
        let m  = HashWrap::<String,ClientInstance>:: new();
        Arc::new(Mutex::new(m))
    };
}

macro_rules! client_instance {
    ($i:expr) =>{
       *CLIENT_MAP.lock().unwrap().get($i).unwrap().client.borrow_mut()
    }
}


pub fn register_client(host : &str, port : i32) -> bool
{
    if ! CLIENT_MAP.lock().unwrap().contains_key(&host.to_string()) {
        let mut url = host.to_string();
        let url = format!("{}:{}", host,port);
        let ci = ClientInstance {
            host : host.to_string(),
            port : port,
            token : "".to_string(),
            client : RefCell::new(wsclient::RequestClient::new(url))
        };
        CLIENT_MAP.lock().unwrap().insert(host.to_string(),ci);
        return true;
    }
    return false;
}


pub fn unregister_client(host : &str) -> bool
{
    if ! CLIENT_MAP.lock().unwrap().contains_key(&host.to_string()) {
        CLIENT_MAP.lock().unwrap().remove(host.to_string());
        return true;
    }
    return false;
}


pub fn client_ok(host : &str) -> bool
{
    if CLIENT_MAP.lock().unwrap().contains_key(&host.to_string()) {
        return true;
    }
    return false;
}

pub fn login(host : & str, username : &str, password : &str ) -> bool {
    let h = host.to_string();
    return client_instance!(&h).login(username, password);
}

pub fn logout(host : & str) -> bool
{
    return unregister_client(host);
}

pub fn ttyd_start(host : & str) -> bool {
    let h = host.to_string();
    if client_ok(host) {
        return client_instance!(&h).ttyd_start();
    }
    return false;
}

pub fn ttyd_stop(host : & str) -> bool {
    let h = host.to_string();
    if client_ok(host) {
        return client_instance!(&h).ttyd_stop();
    }
    return false;
}



#[cfg(test)]
mod tests {
    use super::*;
    use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
    use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
    use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};
    use anyhow;
    use futures::executor::block_on;

    const HOST : &str = "127.0.0.1";
    const PORT : i32 = 5899;
    const USERNAME : &str = "duanwujie";
    const PASSWORD : &str = "linx";

    #[test]
    fn register_client_worked() {
        let state = register_client(HOST,PORT);
        assert_eq!(state,true);
    }

    #[test]
    fn unregister_client_worked() {
        let state = unregister_client(HOST);
        assert_eq!(state,true);
    }


    #[test]
    fn host_login_worked(){
        register_client(HOST,PORT);
        let login_state  = client_instance!(&String::from(HOST)).login(USERNAME,PASSWORD);
        assert_eq!(login_state,true)
    }

    #[test]
    fn ssh_login_worked(){
        register_client(HOST,PORT);
        let login_state  = client_instance!(&String::from(HOST)).ssh_login(USERNAME,PASSWORD);
        assert_eq!(login_state,true);
    }

    #[test]
    fn test_token_worked(){
        register_client(HOST,PORT);
        let login_state  = client_instance!(&String::from(HOST)).ssh_login(USERNAME,PASSWORD);
        let state = client_instance!(&String::from(HOST)).ttyd_start();
        assert_eq!(state,true);
        let state = client_instance!(&String::from(HOST)).ttyd_stop();
        assert_eq!(state,true);

    }


}