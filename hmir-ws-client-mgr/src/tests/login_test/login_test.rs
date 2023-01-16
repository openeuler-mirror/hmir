use crate::{client_instance, tests};
use crate::ws_client_mgr::{register_client, CLIENT_MAP};
use hmir_errno::errno;
use hmir_errno::errno::HMIR_SUCCESS;

use crate::tests::test_default_args::{HOST, PORT, R_PASSWORD, USERNAME};

#[test]
fn host_login_worked(){
    register_client(HOST,PORT);
    let login_state  = client_instance!(&String::from(HOST)).login(USERNAME,R_PASSWORD);
    assert_eq!(login_state,errno::HMIR_SUCCESS)
}