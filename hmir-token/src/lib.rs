
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, Algorithm,EncodingKey, DecodingKey};
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};
use log::{error};

// use crate::hmir_errno::errno;




#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user : String, //The username
    exp  : u64,    //Expiration timestamp (seconds since the Unix epoch)
}

/// Token 默认有效期（秒），可通过环境变量 `HMIR_TOKEN_EXPIRE_SECONDS` 覆盖。
pub const DEFAULT_TOKEN_EXPIRE_SECONDS: u64 = 24 * 60 * 60; // 24 小时


pub const PUBLIC_KEY : &str = "/etc/hmir/hmir-public-key.pem";
pub const PRIVATE_KEY : &str = "/etc/hmir/hmir-private-key.pem";


/// 返回当前 Unix 时间戳（秒）。
fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// 返回 Token 有效期（秒）：优先读取环境变量 `HMIR_TOKEN_EXPIRE_SECONDS`，
/// 缺省或非法（非数字或 <=0）时回退到 [`DEFAULT_TOKEN_EXPIRE_SECONDS`]。
fn token_expire_seconds() -> u64 {
    std::env::var("HMIR_TOKEN_EXPIRE_SECONDS")
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(DEFAULT_TOKEN_EXPIRE_SECONDS)
}


pub fn token_generate(user : &String) -> String
{
    let my_claims = Claims {
        user : user.clone(),
        exp  : now_timestamp() + token_expire_seconds(),
    };

    let exits = std::path::Path::new(PRIVATE_KEY).exists();
    if exits {
        let mut file = std::fs::File::open(PRIVATE_KEY).unwrap();
        let mut private_key = String::new();
        file.read_to_string(&mut private_key).unwrap();

        let token = encode(&Header::new(Algorithm::RS256), &my_claims,
                           &EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap()).unwrap();
        token
    } else {
        error!("Could not find the private key {}",PRIVATE_KEY);
        println!("Could not find the private key {}",PRIVATE_KEY);
        // std::process::exit(-1);
        "".to_string()
    }
}


pub fn _token_verify(token : String) -> bool {
    let mut file = std::fs::File::open(PUBLIC_KEY).unwrap();
    let mut public_key = String::new();
    file.read_to_string(&mut public_key).unwrap();

    let _token_data = decode::<Claims>(&token, &DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(), &Validation::new(Algorithm::RS256));

    match _token_data  {
        Ok(c) => {
            let user = c.claims.user;
            println!("the login user is : {}",user);
            return true;
        },
        Err(err) => match *err.kind() {
            _ => {
                println!("Exception occur : {}",err.to_string());
                return false;
            }
        },
    };
}

pub fn token_verify(token : String) -> bool {
    let exits = std::path::Path::new(PUBLIC_KEY).exists();
    match exits {
        true => {
            return _token_verify(token);
        },
        _ => {
            error!("Could not find the public key {}",PUBLIC_KEY);
            return false;
        }
    }

}


#[macro_export]
macro_rules! TokenChecker {
    ($i:expr) => {
        let verify = hmir_token::token_verify($i);
        if !verify {
            let mut response = HashWrap::<i32,i32>:: new();
            response.error(errno::HMIR_ERR_TOKEN,String::from(errno::HMIR_MSG[errno::HMIR_ERR_TOKEN]));
            let serialized = serde_json::to_string(&response).unwrap();
            return Ok(serialized)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use hmir_errno::errno;

    #[test]
    fn token_genertate_it_works()
    {

        let i = errno::HMIR_SUCCESS;
        let token = token_generate(&String::from("root"));
        println!("{}",token);
        let verify = token_verify(token);
        assert_eq!(verify,true);
    }


}
