
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, Algorithm,EncodingKey, DecodingKey};
use std::io::Read;
use log::{error};

// use crate::hmir_errno::errno;




#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user : String, //The username
    exp  : usize,  //Expiration timestamp
}

pub const EXP_TIME : usize = 4852281600;//2023/10/7


pub const PUBLIC_KEY : &str = "/etc/hmir//hmir-public-key.pem";
pub const PRIVATE_KEY : &str = "/etc/hmir/hmir-private-key.pem";


pub fn token_generate(user : &String) -> String
{
    let my_claims = Claims {
        user : user.clone(),
        exp  : EXP_TIME //Never expiration
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
            error!("Could not find the private key {}",PUBLIC_KEY);
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
