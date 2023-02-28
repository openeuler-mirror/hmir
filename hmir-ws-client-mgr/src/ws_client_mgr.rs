use std::sync::{Arc,Mutex};
use hmir_hash::HashWrap;
use lazy_static::lazy_static;
use std::cell::RefCell;
use hmir_ws_client::ws_client::RequestClient;

pub struct ClientInstance{
    #[allow(dead_code)]
    host : String,
    #[allow(dead_code)]
    port : i32,
    #[allow(dead_code)]
    token : String,
    pub client : RefCell<RequestClient>
}

type ClientType = Arc<Mutex<HashWrap<String,ClientInstance>>>;

lazy_static! {
    pub static ref CLIENT_MAP: ClientType = {
        let m  = HashWrap::<String,ClientInstance>:: new();
        Arc::new(Mutex::new(m))
    };
}

#[macro_export]
macro_rules! client_instance {
    ($i:expr) =>{
       *CLIENT_MAP.lock().unwrap().get($i).unwrap().client.borrow_mut()
    }
}

#[macro_export]
macro_rules! mgr_fn_define {
    ($name:ident) => {
        pub fn $name(host: & str) -> (usize,String){
            let h = host.to_string();
            return client_instance!(&h).$name();
        }
    }
}

#[allow(dead_code)]
pub fn register_client(host : &str, port : i32) -> bool
{
    if ! CLIENT_MAP.lock().unwrap().contains_key(&host.to_string()) {
        let url = format!("{}:{}", host,port);
        let c = RequestClient::new(url);
        match c {
            Ok(client) => {
                let ci = ClientInstance {
                    host : host.to_string(),
                    port : port,
                    token : "".to_string(),
                    client : RefCell::new(client)
                };
                CLIENT_MAP.lock().unwrap().insert(host.to_string(),ci);
                return true;
            }
            Err(_e) => {
                return false;
            }
        }
    }
    return true;
}


pub fn unregister_client(host : &str) -> bool
{
    if CLIENT_MAP.lock().unwrap().contains_key(&host.to_string()) {
        CLIENT_MAP.lock().unwrap().remove(host.to_string());
        return true;
    }
    return false;
}

#[allow(dead_code)]
pub fn client_ok(host : &str) -> bool
{
    if CLIENT_MAP.lock().unwrap().contains_key(&host.to_string()) {
        return true;
    }
    return false;
}