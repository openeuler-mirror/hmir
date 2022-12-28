

use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};
use tokio::runtime::Builder;
use jsonrpsee::rpc_params;
use hmir_hash::HashWrap;


pub struct RequestClient {
    pub client  : Client,
    pub runtime : tokio::runtime::Runtime,
}

impl RequestClient {
    pub fn new(uri : std::string::String) -> Self {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let client = runtime.block_on(async {
            let uri: Uri = format!("ws://{}", uri).parse().unwrap();
            let (tx, rx) = WsTransportClientBuilder::default().build(uri).await.unwrap();
            let client: Client = ClientBuilder::default().build_with_tokio(tx, rx);
            // let response: String = client.request("ttyd-start", None).await.unwrap();
            client
        });

        RequestClient{ client: client, runtime: runtime }
    }

    pub fn ttyd_start(&self) -> bool {
        let state = self.runtime.block_on(async {
            let response: String = self.client.request("ttyd-start", None).await.unwrap();
            let p: HashWrap::<i32,i32> = serde_json::from_str(response.as_str()).unwrap();
            return p.is_success();
        });
        return state;
    }

    pub fn ttyd_stop(&self) -> bool {
        let state = self.runtime.block_on(async {
            let response: String = self.client.request("ttyd-stop", None).await.unwrap();
            let p: HashWrap::<i32,i32> = serde_json::from_str(response.as_str()).unwrap();
            return p.is_success();
        });
        return state;
    }

    pub fn login(&self,username : &str, password : &str ) -> bool {
        let state = self.runtime.block_on(async {
            let response: Result<String, _> = self.client.request("pam-auth", rpc_params![username,password]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<i32,i32> = serde_json::from_str(result.as_str()).unwrap();
                    return p.is_success();
                }
                _ => { return false;}
            }
        });
        return state;
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
    fn login_success_worked(){
        let client = RequestClient::new("172.30.24.123:5898".to_string());
        let login_state = client.login("root","radlcdss");
        assert_eq!(login_state,true)
    }
}