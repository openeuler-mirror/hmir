use hmir_errno::errno;
use hmir_hash::HashWrap;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use ws_client::RequestClient;
use crate::ws_client;

impl RequestClient {

    pub fn update_token(& mut self,token : &String)
    {
        self.token = token.clone();
    }
    
    pub fn login(& mut self,username : &str, password : &str ) -> usize {
        let (state,token) = self.runtime.block_on(async {
            let response: Result<String, _> = self.client.request("pam-auth", rpc_params![username,password]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    if p.is_success() {
                        let token =  p.get(&String::from("token")).unwrap();
                        return (p.code(),token.clone());
                    }else {
                        return (p.code(),String::from(""));
                    }
                }
                _ => {
                    return (errno::HMIR_ERR_COMM,String::from(""));
                }
            }
        });

        if state == errno::HMIR_SUCCESS {
            self.update_token(&token);
        }
        return state;
    }

    pub fn ssh_login(& mut self,username : &str, password: &str) -> usize {
        let (state,token) = self.runtime.block_on( async  {
            let response: Result<String, _> = self.client.request("ssh-auth", rpc_params![username,password]).await;
            match response {
                Ok(result) => {
                    let p:HashWrap<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    if p.is_success() {
                        let token =  p.get(&String::from("token")).unwrap();
                        return (p.code(),token.clone());
                    }else {
                        return (p.code(),String::from(""));
                    }
                }
                _ => { return (errno::HMIR_ERR_COMM,String::from("")) ;}
            }
        });

        if state == errno::HMIR_SUCCESS {
            self.update_token(&token);
        }

        return state;
    }
    
}