//! SSH认证模块，主要用于测试目的
//!
//!
//! - ssh-auth : 使用用户名和密码完成远程认证。
//! 请求格式:
//! ```
//! {
//!    "jsonrpc":"2.0",
//!    "id":1,
//!    "method":"ssh-auth",
//!    "params":["username","password"]
//! }
//! ```
//! 返回格式：
//! ```
//!    {
//!        "code":0,
//!        "result":
//!        {
//!            "token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJ1c2VyIjoiZHVhbnd1amllIiwiZXhwIjo0ODUyMjgxNjAwfQ.QarvhdHuCpp87ipWD8hwN9bQg2dEdUtr5h3ksDu-MUXWeZRzXEef7jVP80n-Aj3fuDdiMglPF2btD_SO_pJnF1YUxipsDHG7cuumPXb4QynEDDtAKVtju5k3pHo9z7PryK7cycVoeNd3CWDR9rvdP0n-jJzBL2W-wN3TCpXH1Tey_NqC25VFZhqofQG3vuuw-XNo4DIwHGPnpJ5EwZ4CJSKgyzIDsciMyfPafBeKFCKRkx-tzX2OZHwF4DSKvnow4ifYC2KsLtdJFM5IewigSxAwdAs7sb4e9cyjNvfqZWfZgprMh8oR5xM_hfFUCUK5YFzDsLSdyzeFs45OL1X4hA"
//!        }
//!    }
//! ```
//! - code为0表示认证成功
//! - 认证成功后，result中会返回对应的token值，该token值用于后续的请求。


use jsonrpsee::ws_server::{RpcModule};

use std::net::TcpStream;
use ssh2::Session;
use hmir_hash::HashWrap;
use hmir_token;
use serde::{Deserialize};
use crate::tokenmgr::register_token;

#[derive(Debug, Clone,Deserialize)]
struct LoginParam {
    username : String,
    password : String
}


macro_rules! ssh_default_result {
    ($i:expr,$j:expr) => {
        let mut response = HashWrap::<String,String>:: new();
        if $i == 0 {
            let token = hmir_token::token_generate($j);
            register_token($j,&token);
            response.insert(String::from("token"),token);
        }
        response.set_code($i);
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}


pub fn is_ssh_auth(username: &str, password: &str) -> bool
{
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    let result = sess.userauth_password(&username, password);
    match result {
        Ok(auth) => {
            return sess.authenticated();
        }
        Err(_e) => {
            return false;
        }
    }
}

pub fn ssh_auth(username : &str, password : &str) -> String

{
    if is_ssh_auth(username,password) {
        ssh_default_result!(0,&String::from(username));
    }
    ssh_default_result!(-1,&String::from(username));
}

#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("ssh-auth", |params, _| {
        let login_param = params.parse::<LoginParam>()?;
        //默认没有error就是成功的
        Ok(ssh_auth(login_param.username.as_str(),login_param.password.as_str()))

    })?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn ssh_connect_it_worked(){

        /*
        {
            "jsonrpc":"2.0",
            "id":1,
            "method":"ssh-auth",
            "params":["duanwujie","linx"]
        }
         */
        let auth = ssh_auth(&"duanwujie".to_string(),&"linx".to_string());

        println!("The auth is : {}",auth);
    }

    #[test]
    fn ssh_wrong_password_it_worked(){
        let auth = ssh_auth(&"root".to_string(),&"noroot".to_string());

        println!("The auth is : {}",auth);
    }
}