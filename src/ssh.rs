



use jsonrpsee::ws_server::{RpcModule};

use std::net::TcpStream;
use ssh2::Session;


#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {


    Ok(())
}


pub fn ssh_auth(username : & String, password: & String) -> bool
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

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn ssh_connect_it_worked(){
        let auth = ssh_auth(&"root".to_string(),&"root".to_string());

        println!("The auth is : {}",auth);
    }

    #[test]
    fn ssh_wrong_password_it_worked(){
        let auth = ssh_auth(&"root".to_string(),&"noroot".to_string());

        println!("The auth is : {}",auth);
    }
}