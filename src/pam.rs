

use pam::Authenticator;


pub fn pam_auth(username: &str, password: &str) -> bool
{

    let service = "hmir";
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



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn pam_auth_it_worked(){
        let auth = pam_auth(&"linx",&"root");
        println!("The auth is : {}",auth);
    }


}