use hmir_errno::errno;
use hmir_hash::HashWrap;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use crate::ws_client::RequestClient;

use jsonrpsee_types::ParamsSer;
use serde_json::json;
use std::collections::BTreeMap;

macro_rules! ExecVirtQuery {
    ($instance:expr, $cmd:expr, $br_info:expr) => {
        let (state, ret_str) = ($instance).runtime.block_on(async {
            let response : Result<String, _> = ($instance).client.request($cmd, Some(ParamsSer::Map($br_info))).await;
            match response {
                Ok(result) =>{
                    let p:HashWrap<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    if p.is_success() {
                        let ret_str =  p.get(&String::from("virt_ret")).unwrap();
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

impl RequestClient {
    #[allow(dead_code)]
    pub fn virt_check_connection(&self) -> (usize, String) {
        let token = json!(self.token.clone());
        let mut br_info = BTreeMap::new();
        br_info.insert("token", token);
        ExecVirtQuery!(self, "virt-check-connection", br_info);
    }
}