use hmir_errno::errno;
use hmir_hash::HashWrap;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use ws_client::RequestClient;
use crate::ws_client;

use jsonrpsee_types::ParamsSer;
use serde_json::json;
use std::collections::BTreeMap;

impl RequestClient {

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
}