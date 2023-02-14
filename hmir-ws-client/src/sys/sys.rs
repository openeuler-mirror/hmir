use hmir_errno::errno;
use hmir_hash::HashWrap;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use ws_client::RequestClient;
use crate::ws_client;

use jsonrpsee_types::ParamsSer;
use serde_json::json;
use hmir_protocol::sys;

impl RequestClient {

    pub fn sys_list_pci_info(&self) -> (usize,String) {
        let token = self.token.clone();
        let state = self.runtime.block_on(async {
            let response: Result<String,_>= self.client.request("sys-list-pci-info", rpc_params![token]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<String,sys::PciDeviceInfo> = serde_json::from_str(result.as_str()).unwrap();
                    return (p.code(),serde_json::to_string(&p.result).unwrap());
                },
                _ => { return (errno::HMIR_ERR_COMM,"".to_string())}
            };
        });
        return state;
    }



}


#[cfg(test)]
mod tests {
    use super::*;

    const URL : &str = "127.0.0.1:5899";

    #[test]
    fn sys_list_pci_info_worked() {
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(mut c) => {
                c.login("root","root");
                let (state,pci_info) = c.sys_list_pci_info();
                println!("{}",pci_info);
            },
            _ => {}
        }
    }

}