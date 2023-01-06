

use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};
use tokio::runtime::Builder;
use jsonrpsee::rpc_params;
use hmir_hash::HashWrap;
// use nix::libc::stat;


pub struct RequestClient {
    pub client  : Client,
    pub token   : String,
    pub runtime : tokio::runtime::Runtime,
}

impl RequestClient {
    pub fn new(uri : std::string::String) -> Result<Self,bool> {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let client = runtime.block_on(async {
            let uri: Uri = format!("ws://{}", uri).parse().unwrap();
            let client_builder = WsTransportClientBuilder::default().build(uri).await;
            match client_builder {
                Ok((tx,rx)) => {
                    let client: Client = ClientBuilder::default().build_with_tokio(tx, rx);
                    Ok(client)
                },
                Err(e) => Err(false)
            }
        });

        match client {
            Ok(c) => {
                Ok(RequestClient{ client: c, token: "".to_string(), runtime: runtime })
            },
            Err(e) => Err(false)
        }
    }


    pub fn ttyd_start(&self) -> bool {
        let token = self.token.clone();
        let state = self.runtime.block_on(async {
            let response: String = self.client.request("ttyd-start", rpc_params![token]).await.unwrap();
            let p: HashWrap::<i32,i32> = serde_json::from_str(response.as_str()).unwrap();
            return p.is_success();
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

    pub fn login(&self,username : &str, password : &str ) -> bool {
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
        return state;
    }

    pub fn ssh_login(& mut self,username : &str, password: &str) -> bool {
        let (state,token) = self.runtime.block_on( async  {
            let response: Result<String, _> = self.client.request("ssh-auth", rpc_params![username,password]).await;
            match response {
                Ok(result) => {
                    let p:HashWrap<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    let token =  p.get(&String::from("token")).unwrap();
                    // println!("The token is : {}",token);
                    return (p.is_success(),token.clone());
                }
                _ => { return (false,String::from("")) ;}
            }
        });

        if state {
            self.update_token(&token);
        }

        return state;
    }

    pub fn update_token(& mut self,token : &String)
    {
        self.token = token.clone();
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
    use serde_json::to_string;

    #[test]
    fn ttyd_start_workd() {
        let client = RequestClient::new("172.30.24.123:5898".to_string());
        client.ttyd_start();
    }

    #[test]
    fn ttyd_stop_worked() {
        let client = RequestClient::new("172.30.24.123:5898".to_string());
        client.ttyd_stop();
    }

    #[test]
    fn login_worked(){
        let client = RequestClient::new("172.30.24.123:5898".to_string());
        let login_state = client.login("root","root");
        assert_eq!(login_state,false)
    }

    #[test]
    fn test_token_worked(){
        let mut client = RequestClient::new("127.0.0.1:5899".to_string());
        let login_state = client.ssh_login("duanwujie","linx");
        assert_eq!(login_state,true);

        let state = client.ttyd_start();
        assert_eq!(state,true);
        let state = client.ttyd_stop();
        assert_eq!(state,true);
    }


    #[test]
    fn login_success_worked(){
        let client = RequestClient::new("172.30.24.123:5898".to_string());
        let login_state = client.login("root","radlcdss");
        assert_eq!(login_state,true)
    }
}