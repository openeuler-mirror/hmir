use hmir_errno::errno;
use hmir_hash::HashWrap;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use crate::ws_client::RequestClient;

use hmir_protocol::pkg;


// use crate::ws_client::client_check;

impl RequestClient {
    #[allow(dead_code)]
    pub fn pkg_all_info(&self) -> (usize, String) {

        client_check!(self.client);

        let token = self.token.clone();
        let (state,info) = self.runtime.block_on(async{
            let response: Result<String, _> = self.client.request("pkg-all-info", rpc_params![token]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<String, pkg::DebPkgInfo> = serde_json::from_str(result.as_str()).unwrap();
                    return (p.code(),serde_json::to_string(&p.result).unwrap());
                },
                _ => { return (errno::HMIR_ERR_COMM,"".to_string())}
            };
        });
        return (state,info);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const URL : &str = "127.0.0.1:5899";

    #[test]
    fn test_pkg_info_worked() {
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(mut c) => {
                c.login("root","root");
                let (state,pkg_info) = c.pkg_all_info();
                println!("{}",pkg_info);
            },
            _ => {}
        }
    }

}