use crate::tests::test_default_args::PORT;
use crate::tests::test_default_args::HOST;
use crate::ws_client_mgr::{register_client, unregister_client};

#[test]
fn register_client_worked() {
    register_client(HOST,PORT);
}

#[test]
fn unregister_client_worked() {
    unregister_client(HOST);
}