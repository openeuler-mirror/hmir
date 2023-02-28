
use hmir_ws_client_mgr::proc::proc_mgr as clientmgr;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn cmd_process_info(host : & str) -> (usize,String) {
    return clientmgr::proc_process_info(host);
}

