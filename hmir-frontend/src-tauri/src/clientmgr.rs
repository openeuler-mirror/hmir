

use crate::wsclient;
use std::sync::{Arc,Mutex};
use hmir_hash::HashWrap;
use crate::wsclient::RequestClient;
use lazy_static::lazy_static;
use std::cell::RefCell;


struct ClientInstance{
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
    static ref CLIENT_MAP: ClientType = {
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

#[allow(dead_code)]
pub fn register_client(host : &str, port : i32) -> bool
{
    if ! CLIENT_MAP.lock().unwrap().contains_key(&host.to_string()) {
        let url = format!("{}:{}", host,port);
        let c = wsclient::RequestClient::new(url);
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
    return false;
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

#[allow(dead_code)]
pub fn login(host : & str, username : &str, password : &str ) -> bool {
    let h = host.to_string();
    return client_instance!(&h).login(username, password);
}


#[allow(dead_code)]
pub fn ssh_login(host : & str, username : &str,password : &str) -> bool {
    let h = host.to_string();
    return client_instance!(&h).ssh_login(username, password);
}

/// ????????????
#[allow(dead_code)]
pub fn logout(host : & str) -> bool
{
    return unregister_client(host);
}

/// ????????????
#[allow(dead_code)]
pub fn ttyd_start(host : & str) -> bool {
    let h = host.to_string();
    if client_ok(host) {
        return client_instance!(&h).ttyd_start();
    }
    return false;
}

/// ????????????
#[allow(dead_code)]
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

    const HOST : &str = "127.0.0.1";
    const PORT : i32 = 5899;
    const USERNAME : &str = "duanwujie";
    const R_PASSWORD : &str = "linx";
    const W_PASSWORD : &str = "wrong_password";

    #[test]
    fn register_client_worked() {
        register_client(HOST,PORT);
    }

    #[test]
    fn unregister_client_worked() {
        unregister_client(HOST);
    }


    #[test]
    fn host_login_worked(){
        register_client(HOST,PORT);
        let login_state  = client_instance!(&String::from(HOST)).login(USERNAME,R_PASSWORD);
        assert_eq!(login_state,true)
    }


    #[test]
    fn host_login_failed_worked(){
        register_client(HOST,PORT);
        let login_state  = client_instance!(&String::from(HOST)).login(USERNAME,W_PASSWORD);
        assert_eq!(login_state,false)
    }


    #[test]
    fn ssh_login_worked(){
        register_client(HOST,PORT);
        let login_state  = client_instance!(&String::from(HOST)).ssh_login(USERNAME,R_PASSWORD);
        assert_eq!(login_state,true);

    }


}