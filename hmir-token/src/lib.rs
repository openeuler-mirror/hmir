extern crate core;

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user : String, //The username
    exp  : usize,  //Expiration timestamp
}

pub fn token_generate(user : String) -> String
{
    let my_claims = Claims {
        user : user,
        exp  : 8640000000000 //Never expiration
    };

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
    println!("The token is : {}",token);

    token
}

pub fn token_verify(token : String) -> bool {
    let _token_data = match decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()) {
        Ok(c) => {
            let user = c.claims.user;
            println!("the login user is : {}",user);
            return true;
        }
        Err(err) => match *err.kind() {
            _ => {
                return false
            }
        },
    };
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_genertate_it_works()
    {
        let token = token_generate(String::from("root"));
        let verify = token_verify(token);
        assert_eq!(verify,true);
    }

}
