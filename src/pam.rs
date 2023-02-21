//! 权限认证模块
//!
//! 支持以下的请求
//! - pam-auth : 查询指定服务状态
//!
//! 请求格式:
//! ```
//! {
//!    "jsonrpc":"2.0",
//!    "id":1,
//!    "method":"pam-auth",
//!    "params":["username","password"]
//! }
//! ```
//! 响应格式:
//!
//! ```
//! {
//!      "jsonrpc": "2.0",
//!      "result": "{\"map\":{\"collectl.service\":{\"name\":\"collectl.service\",\"description\":\"LSB: Collectl monitors system performance.\",\"load_state\":\"Loaded\",\"active_state\":\"Inactive\",\"sub_state\":\"Dead\",\"follow_unit\":null,\"object_path\":\"/org/freedesktop/systemd1/unit/collectl_2eservice\",\"job_id\":0,\"job_ty\":\"\",\"job_object_path\":\"/\"}}}",
//!      "id": 1
//! }
//! ```


use pam::Authenticator;
use jsonrpsee::ws_server::{RpcModule};
use hmir_hash::HashWrap;
use hmir_token;
use hmir_errno::errno;
use serde::{Deserialize};


#[derive(Debug, Clone,Deserialize)]
struct LoginParam {
    username : String,
    password : String
}

macro_rules! pam_default_result {
    ($i:expr,$j:expr) =>{
        let mut response = HashWrap::<String,String>:: new();
        if $i == 0 {
            let token = hmir_token::token_generate($j);
            response.insert(String::from("token"),token);
        }
        response.set_code($i);
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}

pub fn pam_auth(username : &str, password : &str) -> String
{
    if is_pam_auth(username,password) {
        pam_default_result!(0,&String::from(username));
    }
    pam_default_result!(errno::HMIR_ERR_COMM,&String::from(username));
}

pub fn is_pam_auth(username: &str, password: &str) -> bool
{
    let (send, recv) = std::sync::mpsc::channel();
    let user = String::from(username);
    let pass = String::from(password);

    std::thread::spawn(move || {
        let sy_time = std::time::SystemTime::now();
        let mut authenticator = Authenticator::with_password("system-auth")
            .expect("Failed to init PAM client.");
        authenticator.get_handler().set_credentials(user, pass);
        println!("Time cost : {:?}", std::time::SystemTime::now().duration_since(sy_time).unwrap().as_millis());
        if authenticator.authenticate().is_ok() && authenticator.open_session().is_ok() {
            send.send("true").unwrap();
            return true;
        }
        else {
            send.send("false").unwrap();
            return false;
        }
    });

    let r = recv.recv_timeout(std::time::Duration::from_millis(3000));
    match r {
        Ok(msg) => {
            if msg == "true" {
                return true;
            }else {
                return false;
            }
        }
        _ => {
            return false;
        }
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
        let auth = pam_auth(&"duanwujie",&"linx");
        println!("The auth is : {}",auth);
    }

}
