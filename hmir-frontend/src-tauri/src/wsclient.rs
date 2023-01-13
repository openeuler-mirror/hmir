

use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
use tokio::runtime::Builder;
use jsonrpsee::rpc_params;
use hmir_hash::HashWrap;
// use nix::libc::stat;
use log4rs;
use log::{error,info};
use hmir_errno::errno;

use jsonrpsee_types::ParamsSer;
use serde_json::json;
use std::collections::BTreeMap;

macro_rules! ExecOvsQuery {
    ($instance:expr, $cmd:expr, $token:expr) => {
        let (state, ret_str) = ($instance).runtime.block_on(async {
            let response : Result<String, _>= ($instance).client.request($cmd, rpc_params![$token]).await;
            match response {
                Ok(result) => {
                    let p:HashWrap<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    if p.is_success() {
                        let ret_str =  p.get(&String::from("ovs_ret")).unwrap();
                        return (p.get_code(), ret_str.clone());
                    } else {
                        return (p.get_code(), p.get_err());
                    }
                },
                _=>{
                    let err_msg = format!("{} Failed!", $cmd);
                    return (errno::HMIR_ERR_COMM, err_msg);
                }
            }
        });
        return (state, ret_str)
    }
}

macro_rules! ExecVsctlOrOfctl {
    ($instance:expr, $cmd:expr, $br_info:expr) => {
        let (state, ret_str) = ($instance).runtime.block_on(async {
            let response : Result<String, _> = ($instance).client.request($cmd, Some(ParamsSer::Map($br_info))).await;
            match response {
                Ok(result) =>{
                    let p:HashWrap<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    if p.is_success() {
                        let ret_str =  p.get(&String::from("ovs_ret")).unwrap();
                        return (p.get_code(), ret_str.clone());
                    } else {
                        return (p.get_code(), p.get_err());
                    }
                },
                _=> {
                    let err_msg = format!("{} Failed!", $cmd);
                    return (errno::HMIR_ERR_COMM, err_msg);
                }
            }
        });
        return (state, ret_str);
    }
}


#[derive(Debug)]
pub struct RequestClient {
    pub client  : Client,
    pub token   : String,
    pub runtime : tokio::runtime::Runtime,
}


use hmir_systemd::{
    build_blocking_client,
    manager::blocking::{OrgFreedesktopSystemd1Manager},
    models::{Unit,IntoModel},
    SystemdObjectType,
};


impl RequestClient {
    pub fn new(uri : std::string::String) -> Result<Self,bool> {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let client = runtime.block_on(async {
            let uri: Uri = format!("ws://{}", uri).parse().unwrap();
            let client_builder = WsTransportClientBuilder::default().build(uri.clone()).await;
            match client_builder {
                Ok((tx,rx)) => {
                    let client: Client = ClientBuilder::default().build_with_tokio(tx, rx);
                    Ok(client)
                },
                Err(_e) => {
                    error!("Connect the remote {} failed, Reason : {}",uri.clone(),_e.to_string());
                    Err(false)
                }
            }
        });

        match client {
            Ok(c) => {
                Ok(RequestClient{ client: c, token: "".to_string(), runtime: runtime })
            },
            Err(_e) => Err(false)
        }
    }


    pub fn ttyd_start(&self) -> bool {
        let token = self.token.clone();
        let state = self.runtime.block_on(async {
            let response: Result<String,_>= self.client.request("ttyd-start", rpc_params![token]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<i32, i32> = serde_json::from_str(result.as_str()).unwrap();
                    return p.is_success();
                },
                Err(_e) => {
                    return false;
                }
            }
        });
        return state;
    }

    pub fn ttyd_stop(&self) -> bool {
        let token = self.token.clone();
        let state = self.runtime.block_on(async {

            let response: String = self.client.request("ttyd-stop", rpc_params![token]).await.unwrap();
            let p: HashWrap::<i32,i32> = serde_json::from_str(response.as_str()).unwrap();
            return p.is_success();
        });
        return state;
    }

    pub fn login(& mut self,username : &str, password : &str ) -> usize {
        let (state,token) = self.runtime.block_on(async {
            let response: Result<String, _> = self.client.request("pam-auth", rpc_params![username,password]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    if p.is_success() {
                        let token =  p.get(&String::from("token")).unwrap();
                        return (p.code(),token.clone());
                    }else {
                        return (p.code(),String::from(""));
                    }
                }
                _ => {
                    return (errno::HMIR_ERR_COMM,String::from(""));
                }
            }
        });

        if state == errno::HMIR_SUCCESS {
            self.update_token(&token);
        }
        return state;
    }

    pub fn ssh_login(& mut self,username : &str, password: &str) -> usize {
        let (state,token) = self.runtime.block_on( async  {
            let response: Result<String, _> = self.client.request("ssh-auth", rpc_params![username,password]).await;
            match response {
                Ok(result) => {
                    let p:HashWrap<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    if p.is_success() {
                        let token =  p.get(&String::from("token")).unwrap();
                        return (p.code(),token.clone());
                    }else {
                        return (p.code(),String::from(""));
                    }
                }
                _ => { return (errno::HMIR_ERR_COMM,String::from("")) ;}
            }
        });

        if state == errno::HMIR_SUCCESS {
            self.update_token(&token);
        }

        return state;
    }

    #[allow(dead_code)]
    pub fn reg_observer(&mut self, url:&str,cmd:u32,duration: u64) ->String {
        let token = self.token.clone();
        let response = self.runtime.block_on( async {
            let response: String = self.client.request("register-observer", rpc_params![token,url,cmd,duration]).await.unwrap();
            return response;
        });
        return response;
    }

    pub fn update_token(& mut self,token : &String)
    {
        self.token = token.clone();
    }

    pub fn ovs_query_connection(&self) -> (usize, String) {
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-connection", token);
    }

    pub fn ovs_query_ports(&self) -> (usize,String){
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-ports", token);
    }

    pub fn ovs_query_bridges(&self) -> (usize,String){
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-bridges", token);
    }

    #[allow(dead_code)]
    pub fn ovs_query_interfaces(&self) -> (usize,String){
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-interfaces", token);
    }

    #[allow(dead_code)]
    pub fn ovs_query_netflow(&self) -> (usize,String) {
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-netflows", token);
    }

    #[allow(dead_code)]
    pub fn ovs_query_ipfix(&self) -> (usize,String) {
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-ipfix", token);
    }

    pub fn ovs_vsctl_add_br(&self, br_name:&str) -> (usize, String)
    {
        let token = json!(self.token.clone());
        let mut br_info = BTreeMap::new();
        br_info.insert("br_name", json!(br_name));
        br_info.insert("token", token);

        ExecVsctlOrOfctl!(self, "ovs-vsctl-add-br", br_info);
    }

    pub fn ovs_vsctl_del_br(&self, br_name:&str) -> (usize, String)
    {
        let token = json!(self.token.clone());
        let mut br_info = BTreeMap::new();
        br_info.insert("br_name", json!(br_name));
        br_info.insert("token", token);

        ExecVsctlOrOfctl!(self, "ovs-vsctl-del-br", br_info);
    }

    pub fn ovs_ofctl_forbid_dstip(&self, br_name:&str, dst_ip:&str, in_port:&str) -> (usize, String)
    {
        let token = json!(self.token.clone());
        let mut br_info = BTreeMap::new();
        br_info.insert("br_name", json!(br_name));
        br_info.insert("dst_ip", json!(dst_ip));
        br_info.insert("in_port", json!(in_port));
        br_info.insert("token", token);

        ExecVsctlOrOfctl!(self, "ovs-ofctl-forbid-dstip", br_info);
    }


    pub fn service_all(&self) -> (usize,String) {
        let token = self.token.clone();

        let (state,service) = self.runtime.block_on(async{
            let response: Result<String, _> = self.client.request("service-all", rpc_params![token]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<String,Unit> = serde_json::from_str(result.as_str()).unwrap();
                    return (p.code(),serde_json::to_string(&p.result).unwrap());
                },
                _ => { return (errno::HMIR_ERR_COMM,"".to_string())}
            };
        });
        return (state,service);
    }

    pub fn service_list_disabled(&self) -> (usize,String) {
        let token = self.token.clone();
        let (state,service) = self.runtime.block_on(async{
            let response: Result<String, _> = self.client.request("service-list-disabled", rpc_params![token]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<String,Unit> = serde_json::from_str(result.as_str()).unwrap();
                    return (p.code(),serde_json::to_string(&p.result).unwrap());
                },
                _ => { return (errno::HMIR_ERR_COMM,"".to_string())}
            };
        });
        return (state,service);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    const URL : &str = "127.0.0.1:5899";


    #[test]
    fn ttyd_start_workd() {
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let state = c.ttyd_start();
                assert_eq!(state,true);
            },
            _ => {}
        }
    }

    #[test]
    fn ttyd_stop_worked() {
        let client = RequestClient::new(String::from(URL));
        client.unwrap().ttyd_stop();
    }

    #[test]
    fn login_worked() {
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(mut c) => {
                let login_state = c.login("root", "root");
                assert_eq!(login_state, 0)
            }
            _ => {}
        }
    }

    #[test]
    fn test_token_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(mut c) => {
                let login_state = c.ssh_login("duanwujie","linx");
                assert_eq!(login_state,errno::HMIR_SUCCESS);
                let state = c.ttyd_start();
                assert_eq!(state,true);
                let state = c.ttyd_stop();
                assert_eq!(state,true);
            },
            _ => {}
        }
    }


    #[test]
    fn login_success_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(mut c) => {
                let login_state = c.login("root","radlcdss");
                assert_eq!(login_state,errno::HMIR_SUCCESS)
            }
            _ => {}
        }
    }

    #[test]
    fn ovs_query_connection_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (state ,_) = c.ovs_query_connection();
                assert_eq!(state, errno::HMIR_SUCCESS);
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_query_ports_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (state ,_)= c.ovs_query_ports();
                assert_eq!(state, errno::HMIR_SUCCESS);
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_query_bridges_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (state,_) = c.ovs_query_bridges();
                assert_eq!(state, errno::HMIR_SUCCESS);
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_query_interfaces_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (state,_) = c.ovs_query_interfaces();
                assert_eq!(state, errno::HMIR_SUCCESS)
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_query_netflow_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (state,_)= c.ovs_query_netflow();
                assert_eq!(state, errno::HMIR_SUCCESS)
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_query_ipfix_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (state,_) = c.ovs_query_ipfix();
                assert_eq!(state, errno::HMIR_SUCCESS)
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_vsctl_add_br_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (state ,_) = c.ovs_vsctl_add_br("br-ckxu");
                assert_eq!(state, errno::HMIR_SUCCESS)
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_vsctl_del_br_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (state ,_) = c.ovs_vsctl_add_br("br-ckxu");
                if state == errno::HMIR_SUCCESS {
                    let (del_state,_) = c.ovs_vsctl_del_br("br-ckxu");
                    assert_eq!(del_state, errno::HMIR_SUCCESS)
                }
            }
            _ => {}
        }
    } 

    #[test]
    fn server_all_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (result,state) = c.service_all();
            }
            _ => {}
        }
    }


    #[test]
    fn server_list_disabled_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let (result,state) = c.service_list_disabled();
                println!("{}",result);
            }
            _ => {}
        }
    }
}