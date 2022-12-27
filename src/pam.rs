

use pam::Authenticator;
use jsonrpsee::ws_server::{RpcModule};
use hmir_hash::HashWrap;
use serde::{Deserialize};


#[derive(Debug, Clone,Deserialize)]
struct LoginParam {
    username : String,
    password : String
}

macro_rules! pam_default_result {
    ($i:expr) =>{
        let mut response = HashWrap::<i32,i32>:: new();
        response.set_code($i);
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}

pub fn pam_auth(username : &str, password : &str) -> String
{
    if is_pam_auth(username,password) {
        pam_default_result!(0);
    }
    pam_default_result!(-1);
}

pub fn is_pam_auth(username: &str, password: &str) -> bool
{
    let mut authenticator = Authenticator::with_password("system-auth")
        .expect("Failed to init PAM client.");
    authenticator.get_handler().set_credentials(username, password);
    if authenticator.authenticate().is_ok() && authenticator.open_session().is_ok() {
        return true;
    }
    else {
        return false;
    }
}


#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("pam-auth", |params, _| {
        let login_param = params.parse::<LoginParam>()?;
        //默认没有error就是成功的
        Ok(pam_auth(login_param.username.as_str(),login_param.password.as_str()))
    })?;

    Ok(())
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn pam_auth_it_worked(){
        let auth = pam_auth(&"linx",&"root");
        println!("The auth is : {}",auth);
    }

}