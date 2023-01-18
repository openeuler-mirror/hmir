use ws_client::RequestClient;
use crate::ws_client;
use hmir_errno::errno;

const URL : &str = "127.0.0.1:5899";

#[test]
fn virt_check_connection_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state ,_) = c.virt_check_connection();
            assert_eq!(state, errno::HMIR_SUCCESS);
        }
        _ => {}
    }
}