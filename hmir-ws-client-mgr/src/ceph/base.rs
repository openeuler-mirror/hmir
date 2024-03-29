use hmir_hash::hmir_result::HmirResult;
use crate::client_instance;
use crate::ws_client_mgr::CLIENT_MAP;

#[allow(dead_code)]
pub fn get_ceph_status(host : & str) -> HmirResult<String> {
    let h = host.to_string();
    return client_instance!(&h).ceph_status();
}
