



use jsonrpsee::ws_server::{RpcModule};

use std::net::TcpStream;
use ssh2::Session;
use hmir_hash::HashWrap;
use hmir_token;
use serde::{Deserialize};

#[derive(Debug, Clone,Deserialize)]
struct LoginParam {
    username : String,
    password : String
}


macro_rules! ssh_default_result {
    ($i:expr,$j:expr) =>{
        let mut response = HashWrap::<String,String>:: new();
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
    todo!()
}

#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("pam-auth", |params, _| {
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
        let auth = ssh_auth(&"duanwujie".to_string(),&"linx".to_string());

        println!("The auth is : {}",auth);
    }

    #[test]
    fn ssh_wrong_password_it_worked(){
        let auth = ssh_auth(&"root".to_string(),&"noroot".to_string());

        println!("The auth is : {}",auth);
    }
}