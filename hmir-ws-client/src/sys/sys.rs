use hmir_errno::errno;
use hmir_hash::HashWrap;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use ws_client::RequestClient;
use crate::ws_client;

use hmir_protocol::sys;

impl RequestClient {

    pub fn client_is_available(&self) -> bool {
        return self.client.is_connected()
    }

    pub fn sys_list_pci_info(&self) -> (usize,String) {

        client_check!(self.client);

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

    pub fn sys_os_all_info(&self) -> (usize,String) {

        client_check!(self.client);

        let token = self.token.clone();
        let state = self.runtime.block_on(async {
            let response: Result<String,_>= self.client.request("sys-os-all-info", rpc_params![token]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<String,sys::SystemAllInfo> = serde_json::from_str(result.as_str()).unwrap();
                    return (p.code(),serde_json::to_string(&p.result).unwrap());
                },
                _ => { return (errno::HMIR_ERR_COMM,"".to_string())}
            };
        });
        return state;
    }

    pub fn sys_set_hostname(&self, pretty_name : String, static_name : String) -> (usize,String)
    {
        client_check!(self.client);

        let token = self.token.clone();
        let state = self.runtime.block_on(async {
            let response: Result<String,_>= self.client.request("sys-set-hostname", rpc_params![token,pretty_name,static_name]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<i32,i32> = serde_json::from_str(result.as_str()).unwrap();
                    return (p.code(),serde_json::to_string(&p.result).unwrap());
                },
                _ => { return (errno::HMIR_ERR_COMM,"".to_string())}
            };
        });
        return state;
    }

    pub fn sys_set_date(&self, date : String) -> (usize,String)
    {
        client_check!(self.client);

        let token = self.token.clone();
        let state = self.runtime.block_on(async {
            let response: Result<String,_>= self.client.request("sys-set-date", rpc_params![token,date]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<i32,i32> = serde_json::from_str(result.as_str()).unwrap();
                    return (p.code(),serde_json::to_string(&p.result).unwrap());
                },
                _ => { return (errno::HMIR_ERR_COMM,"".to_string())}
            };
        });
        return state;
    }

    pub fn sys_get_date(&self) -> (usize,String){
        client_check!(self.client);
        let token = self.token.clone();
        let state = self.runtime.block_on(async {
            let response: Result<String,_>= self.client.request("sys-get-date", rpc_params![token]).await;
            match response {
                Ok(result) => {
                    let p: HashWrap::<i32,i32> = serde_json::from_str(result.as_str()).unwrap();
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

    const URL : &str = "172.30.21.13:5899";

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

    #[test]
    fn sys_os_all_info_worked() {
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(mut c) => {
                c.login("root","root");
                let (state,pci_info) = c.sys_os_all_info();
                println!("{}",pci_info);
            },
            _ => {}
        }
    }

    #[test]
    fn test_sys_set_hostname_worked() {
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(mut c) => {
                c.login("root","root");
                let (state,pci_info) = c.sys_set_hostname("I am developer".to_string(),"dwj".to_string());

                println!("{}",errno::HMIR_MSG[state]);
            },
            _ => {}
        }
    }

    #[test]
    fn test_sys_set_date_worked() {
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(mut c) => {
                c.login("root","root");
                let (state,pci_info) = c.sys_set_date("2023-02-23 15:19:00".to_string());

                println!("{}",errno::HMIR_MSG[state]);
            },
            _ => {}
        }
    }

    #[test]
    fn test_sys_get_date_worked() {
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(mut c) => {
                c.login("root","root");
                let (state,pci_info) = c.sys_get_date();
                println!("{}",errno::HMIR_MSG[state]);
            },
            _ => {}
        }
    }
}