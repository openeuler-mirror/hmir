use hmir_errno::errno;
use hmir_hash::HashWrap;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use ws_client::RequestClient;
use crate::ws_client;

use hmir_hash::hmir_result::HmirResult;

impl RequestClient {

    pub fn ceph_status(& mut self) -> HmirResult<String> {
        // let (state, content) = self.runtime.block_on(async {
        let ret = self.runtime.block_on(async {
            let response: Result<HmirResult<String>, _> = self.client.request("ceph-status", rpc_params![]).await;
            match response {
                Ok(result) => {
                    result
                    // if result.is_success() {
                    //     (result.code(), result.result)
                    // } else {
                    //     (result.code(), result.errmsg)
                    // }
                }
                Err(e) => {
                    HmirResult::new(errno::HMIR_ERR_COMM, 
                                    String::from(format!("Err: {}", e.to_string())), 
                                    String::from(""))
                    // return (errno::HMIR_ERR_COMM, String::from(format!("Err: {}", e.to_string())));
                }
            }
        });

        // (state,content)
        ret
    }

}