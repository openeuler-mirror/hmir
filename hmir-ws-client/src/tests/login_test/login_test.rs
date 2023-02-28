#[cfg(test)]
mod tests {
    use crate::ws_client::RequestClient;

    #[test]
    fn login_worked() {
        let client = RequestClient::new(String::from(URL));
        match client {
            Ok(mut c) => {
                let login_state = c.login("root", "root");
                assert_eq!(login_state, 0)
            }
            _ => {}
        }
    }
}