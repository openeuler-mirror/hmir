use hmir_errno::errno;
use hmir_hash::HashWrap;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use ws_client::RequestClient;
use crate::ws_client;

impl RequestClient {

    pub fn ceph_status(& mut self) -> String {
        let (state,content) = self.runtime.block_on(async {
            let response: Result<String, _> = self.client.request("ceph-status", rpc_params![]).await;
            // println!("response: {:?}", response);
            match response {
                Ok(result) => {
                    // let p: HashWrap::<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    (0, result)
                    // if p.is_success() {
                    //     let token =  p.get(&String::from("token")).unwrap();
                    //     return (p.code(),token.clone());
                    // }else {
                    //     return (p.code(),String::from(""));
                    // }
                }
                _ => {
                    return (errno::HMIR_ERR_COMM,String::from(""));
                }
            }
        });
        
        return content;
    }

}