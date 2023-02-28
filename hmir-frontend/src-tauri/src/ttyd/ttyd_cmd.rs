
use hmir_ws_client_mgr::ttyd::ttyd_mgr as clientmgr;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn cmd_ttyd_start(host : & str) -> (usize,String) {
    return clientmgr::ttyd_start(host);
}

#[tauri::command]
pub fn cmd_ttyd_stop(host : & str) -> (usize,String) {
    return clientmgr::ttyd_stop(host);
}