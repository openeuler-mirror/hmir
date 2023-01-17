use ws_client::RequestClient;
use crate::ws_client;
use crate::tests::test_default_args::URL;


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