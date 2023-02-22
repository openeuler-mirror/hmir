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

    pub fn ttyd_start(&self) ->  (usize,String) {

        let token = self.token.clone();
        let state = self.runtime.block_on(async {
            let response: Result<String,_>= self.client.request("ttyd-start", rpc_params![token]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<i32, i32> = serde_json::from_str(result.as_str()).unwrap();
                    return (p.code(),serde_json::to_string(&p.result).unwrap());
                },
                _ => {
                    return (errno::HMIR_ERR_COMM,"".to_string())
                }
            }
        });
        return state;
    }

    pub fn ttyd_stop(&self) ->  (usize,String)  {

        // client_check!(self.client);

        let token = self.token.clone();
        let state= self.runtime.block_on(async {

            let response: Result<String,_>= self.client.request("ttyd-stop", rpc_params![token]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<i32, i32> = serde_json::from_str(result.as_str()).unwrap();
                    return (p.code(),serde_json::to_string(&p.result).unwrap());
                },
                _ => {
                    return (errno::HMIR_ERR_COMM,"".to_string())
                }
            }
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