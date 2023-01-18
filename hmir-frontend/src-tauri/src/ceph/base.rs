use hmir_hash::hmir_result::HmirResult;
use hmir_ws_client_mgr::ceph::base as clientmgr;

#[tauri::command]
pub fn cmd_get_ceph_status(host: &str) -> HmirResult<String> {
    return clientmgr::get_ceph_status(host);
}
