

use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
use tokio::runtime::Builder;
use jsonrpsee::rpc_params;
use hmir_hash::HashWrap;
// use nix::libc::stat;
use log4rs;
use log::{error,info};

#[derive(Debug)]
pub struct RequestClient {
    pub client  : Client,
    pub token   : String,
    pub runtime : tokio::runtime::Runtime,
}

impl RequestClient {
    pub fn new(uri : std::string::String) -> Result<Self,bool> {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let client = runtime.block_on(async {
            let uri: Uri = format!("ws://{}", uri.clone()).parse().unwrap();
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
            Err(_e) => {
                Err(false)
            }
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

    pub fn login(& mut self,username : &str, password : &str ) -> bool {
        let (state,token) = self.runtime.block_on(async {
            let response: Result<String, _> = self.client.request("pam-auth", rpc_params![username,password]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    if p.is_success() {
                        let token =  p.get(&String::from("token")).unwrap();
                        return (p.is_success(),token.clone());
                    }else {
                        return (false,String::from(""));
                    }
                }
                _ => {
                    return (false,String::from(""));
                }
            }
        });

        if state {
            self.update_token(&token);
        }
        return state;
    }

    pub fn ssh_login(& mut self,username : &str, password: &str) -> bool {
        let (state,token) = self.runtime.block_on( async  {
            let response: Result<String, _> = self.client.request("ssh-auth", rpc_params![username,password]).await;
            match response {
                Ok(result) => {
                    let p:HashWrap<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    if p.is_success() {
                        let token =  p.get(&String::from("token")).unwrap();
                        return (p.is_success(),token.clone());
                    }else {
                        return (false,String::from(""));
                    }                }
                _ => { return (false,String::from("")) ;}
            }
        });

        if state {
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

    #[allow(dead_code)]
    pub fn ovs_query_connection(&self) -> bool{
        let token = self.token.clone();
        let state = self.runtime.block_on(async {

            let response: String = self.client.request("ovs-query-connection", rpc_params![token]).await.unwrap();
            let p: HashWrap::<i32,i32> = serde_json::from_str(response.as_str()).unwrap();
            return p.is_success();
        });
        return state;
    }

    #[allow(dead_code)]
    pub fn ovs_query_ports(&self) -> bool{
        let token = self.token.clone();
        let state = self.runtime.block_on(async {

            let response: String = self.client.request("ovs-query-ports", rpc_params![token]).await.unwrap();
            let p: HashWrap::<i32,i32> = serde_json::from_str(response.as_str()).unwrap();
            return p.is_success();
        });
        return state;
    }

    #[allow(dead_code)]
    pub fn ovs_query_bridges(&self) -> bool{
        let token = self.token.clone();
        let state = self.runtime.block_on(async {

            let response: String = self.client.request("ovs-query-bridges", rpc_params![token]).await.unwrap();
            let p: HashWrap::<i32,i32> = serde_json::from_str(response.as_str()).unwrap();
            return p.is_success();
        });
        return state;
    }

    #[allow(dead_code)]
    pub fn ovs_query_interfaces(&self) -> bool{
        let token = self.token.clone();
        let state = self.runtime.block_on(async {

            let response: String = self.client.request("ovs-query-interfaces", rpc_params![token]).await.unwrap();
            let p: HashWrap::<i32,i32> = serde_json::from_str(response.as_str()).unwrap();
            return p.is_success();
        });
        return state;
    }

    #[allow(dead_code)]
    pub fn ovs_query_netflow(&self) -> bool{
        let token = self.token.clone();
        let state = self.runtime.block_on(async {

            let response: String = self.client.request("ovs-query-netflow", rpc_params![token]).await.unwrap();
            let p: HashWrap::<i32,i32> = serde_json::from_str(response.as_str()).unwrap();
            return p.is_success();
        });
        return state;
    }

    #[allow(dead_code)]
    pub fn ovs_query_ipfix(&self) -> bool{
        let token = self.token.clone();
        let state = self.runtime.block_on(async {

            let response: String = self.client.request("ovs-query-ipfix", rpc_params![token]).await.unwrap();
            let p: HashWrap::<i32,i32> = serde_json::from_str(response.as_str()).unwrap();
            return p.is_success();
        });
        return state;
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
                assert_eq!(login_state, false)
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
                assert_eq!(login_state,true);
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
                assert_eq!(login_state,true)
            }
            _ => {}
        }
    }

    #[test]
    fn ovs_query_connection_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let state = c.ovs_query_connection();
                assert_eq!(state, true)
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_query_ports_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let state = c.ovs_query_ports();
                assert_eq!(state, true)
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_query_bridges_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let state = c.ovs_query_bridges();
                assert_eq!(state, true)
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_query_interfaces_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let state = c.ovs_query_interfaces();
                assert_eq!(state, true)
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_query_netflow_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let state = c.ovs_query_netflow();
                assert_eq!(state, true)
            }
            _ => {}
        }
    } 

    #[test]
    fn ovs_query_ipfix_worked(){
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(c) => {
                let state = c.ovs_query_ipfix();
                assert_eq!(state, true)
            }
            _ => {}
        }
    } 

}