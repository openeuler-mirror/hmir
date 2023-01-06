

use crate::wsclient;
use std::sync::{Arc,Mutex};
use hmir_hash::HashWrap;
use crate::wsclient::RequestClient;
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
        let m  = HashWrap::<String, ClientInstance>:: new();
        Arc::new(Mutex::new(m))
    };
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

pub fn unregister_client(host : &str, port : i32) -> bool
{
    if ! CLIENT_MAP.lock().unwrap().contains_key(&host.to_string()) {
        CLIENT_MAP.lock().unwrap().remove(host.to_string());
        return true;
    }
    return false;
}

macro_rules! client_instance {
    ($i:expr) =>{
       *CLIENT_MAP.lock().unwrap().get($i).unwrap().client.borrow_mut()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
    use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
    use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};
    use anyhow;
    use futures::executor::block_on;

    #[test]
    fn register_client_worked() {
        register_client("172.30.24.123",5898);
    }

    #[test]
    fn unregister_client_worked() {
        unregister_client("172.30.24.123",5898);
    }


    #[test]
    fn host_login_worked(){
        let host = "172.30.24.123".to_string();
        register_client("172.30.24.123",5898);
        let login_state  = client_instance!(&host).login("root","radlcdss");
        assert_eq!(login_state,true)
    }

    #[test]
    fn ssh_login_worked(){
        let host = "127.0.0.1".to_string();
        register_client("127.0.0.1",5899);
        let login_state  = client_instance!(&host).ssh_login("duanwujie","linx");
        assert_eq!(login_state,true);

    }


}