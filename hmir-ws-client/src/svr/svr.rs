

use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
use tokio::runtime::Builder;
use jsonrpsee::rpc_params;
use hmir_hash::HashWrap;
// use nix::libc::stat;
use log4rs;
use log::{error,info};
use hmir_errno::errno;

use serde_json::json;
use std::collections::BTreeMap;
use crate::ws_client::RequestClient;

use hmir_systemd::{
    build_blocking_client,
    manager::blocking::{OrgFreedesktopSystemd1Manager},
    models::{HmirUnit,Unit,IntoModel},
    SystemdObjectType,
};


impl RequestClient {

    fn _svr_get_unit(&self,cmd: &str) -> (usize,String) {
        let token = self.token.clone();
        let (state,service) = self.runtime.block_on(async{
            let response: Result<String, _> = self.client.request(cmd, rpc_params![token]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<String,HmirUnit> = serde_json::from_str(result.as_str()).unwrap();
                    return (p.code(),serde_json::to_string(&p.result).unwrap());
                },
                _ => { return (errno::HMIR_ERR_COMM,"".to_string())}
            };
        });
        return (state,service);
    }

    pub fn svr_list_enabled_service(&self) -> (usize,String) {
        self._svr_get_unit("svr-list-enabled-service")
    }

    pub fn svr_list_disabled_service(&self) -> (usize,String) {
        self._svr_get_unit("svr-list-disabled-service")
    }

    pub fn svr_list_static_service(&self) -> (usize,String) {
        self._svr_get_unit("svr-list-static-service")
    }

    pub fn svr_list_enabled_timer(&self) -> (usize,String)
    {
        self._svr_get_unit("svr-list-enabled-timer")
    }

    pub fn svr_list_disabled_timer(&self) -> (usize,String)
    {
        self._svr_get_unit("svr-list-disabled-timer")
    }

    pub fn svr_list_static_timer(&self) -> (usize,String) {
        self._svr_get_unit("svr-list-static-timer")
    }

    pub fn svr_list_enabled_socket(&self) -> (usize,String) {
        self._svr_get_unit("svr-list-enabled-socket")
    }

    pub fn svr_list_disabled_socket(&self) -> (usize,String) {
        self._svr_get_unit("svr-list-disabled-socket")
    }

    pub fn svr_list_static_socket(&self) -> (usize,String) {
        self._svr_get_unit("svr-list-static-socket")
    }

    pub fn svr_list_enabled_target(&self) -> (usize,String) {
        self._svr_get_unit("svr-list-enabled-target")
    }

    pub fn svr_list_disabled_target(&self) -> (usize,String) {
        self._svr_get_unit("svr-list-disabled-target")
    }

    pub fn svr_list_static_target(&self) -> (usize,String) {
        self._svr_get_unit("svr-list-static-target")
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    const URL : &str = "127.0.0.1:5899";

    // macro_rules! svr_print_result {
    //     ($i:expr) =>{
    //                 let p: HashWrap::<String,HmirUnit> = serde_json::from_str(i.as_str()).unwrap();
    //
    //
    //     }
    // }

    #[test]
    fn svr_list_enable_service_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (result,state) = c.svr_list_enabled_service();
            }
            _ => {}
        }
    }


    #[test]
    fn server_list_disabled_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (result,state) = c.svr_list_disabled_service();
                println!("{}",result);
            }
            _ => {}
        }
    }


    #[test]
    fn svr_list_enabled_timer_worked(){
        let client = RequestClient::new(String::from(URL));


        match client {
            Ok(mut c) => {
                c.login("root","root");
                let (result,state) = c.svr_list_enabled_timer();
                println!("{}",result);
            }
            _ => {}
        }
    }

    #[test]
    fn svr_list_enabled_socket_worked(){
        let client = RequestClient::new(String::from(URL));


        match client {
            Ok(mut c) => {
                c.login("root","root");
                let (result,state) = c.svr_list_enabled_socket();
                println!("{}",result);
            }
            _ => {}
        }
    }



    #[test]
    fn svr_list_disabled_socket_worked(){
        let client = RequestClient::new(String::from(URL));


        match client {
            Ok(mut c) => {
                c.login("root","root");
                let (result,state) = c.svr_list_disabled_socket();
                println!("{}",result);
            }
            _ => {}
        }
    }



    #[test]
    fn svr_list_static_socket_worked(){
        let client = RequestClient::new(String::from(URL));


        match client {
            Ok(mut c) => {
                c.login("root","root");
                let (state,result) = c.svr_list_static_socket();
                println!("{}",result);
            }
            _ => {}
        }
    }


    #[test]
    fn svr_all_worked(){
        let client = RequestClient::new(String::from(URL));


        match client {
            Ok(mut c) => {
                c.login("root","root");
                // let (state,result) = c.svr_list_enabled_service();
                // println!("{}",result);
                // let (state,result) = c.svr_list_disabled_service();
                // println!("{}",result);
                // let (state,result) = c.svr_list_static_service();
                // println!("{}",result);


                let (state,result) = c.svr_list_enabled_socket();
                println!("{}",result);
                let (state,result) = c.svr_list_disabled_socket();
                println!("{}",result);
                let (state,result) = c.svr_list_static_socket();
                println!("{}",result);


                // let (state,result) = c.svr_list_enabled_timer();
                // println!("{}",result);
                // let (state,result) = c.svr_list_disabled_timer();
                // println!("{}",result);
                // let (state,result) = c.svr_list_static_timer();
                // println!("{}",result);
                //
                //
                //
                // let (state,result) = c.svr_list_enabled_target();
                // println!("{}",result);
                // let (state,result) = c.svr_list_disabled_target();
                // println!("{}",result);
                // let (state,result) = c.svr_list_static_target();
                // println!("{}",result);


            }
            _ => {}
        }
    }
}