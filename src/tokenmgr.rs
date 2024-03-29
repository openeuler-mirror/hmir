use std::sync::{Arc, Mutex};
use hmir_hash::HashWrap;

pub struct UserInfo {
    #[allow(dead_code)]
    user : String,
    token : String,
}

type UserInfoType = Arc<Mutex<HashWrap<String,UserInfo>>>;

lazy_static! {
    static ref TOKEN_MAP: UserInfoType = {
        let m  = HashWrap::<String,UserInfo>:: new();
        Arc::new(Mutex::new(m))
    };
}

#[allow(dead_code)]
pub fn register_token(user : &String,token : &String) -> bool
{
    if TOKEN_MAP.lock().unwrap().contains_key(user) {
        return false;
    }
    let userinfo =  UserInfo { user : user.clone() , token : token.clone(), };
    TOKEN_MAP.lock().unwrap().insert(user.clone(),userinfo);
    return true;
}


#[allow(dead_code)]
pub fn unregister_token(user : &String, token : &String) -> bool
{
    if TOKEN_MAP.lock().unwrap().contains_key(user){
        let stored_token = TOKEN_MAP.lock().unwrap().get(user).unwrap().token.clone();
        if stored_token.eq(token) {
            TOKEN_MAP.lock().unwrap().remove(user.clone());
            return true;
        }
        return false;
    }
    return false;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_token_worked()
    {
        let user = String::from("root");
        let token = hmir_token::token_generate(&user);
        let state = register_token(&user,&token);
        assert_eq!(state,true);
        let stored_token = TOKEN_MAP.lock().unwrap().get(&user).unwrap().token.clone();
        println!("The gen token is : {}",stored_token);

        let state = unregister_token(&user,&stored_token);
        assert_eq!(state,true);
    }


}

