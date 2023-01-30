use crate::{client_instance, tests};
use crate::ws_client_mgr::{register_client, CLIENT_MAP};
use hmir_errno::errno;
use hmir_errno::errno::HMIR_SUCCESS;

use crate::tests::test_default_args::{HOST, PORT};

use crate::ceph::base::get_ceph_status;

#[test]
fn get_ceph_status_test(){
    register_client(HOST,PORT);
    let ret  = get_ceph_status(HOST);
    println!("ret: {:?}", ret);
}