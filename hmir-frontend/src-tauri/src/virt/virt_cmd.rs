use hmir_ws_client_mgr::virt::virt_mgr as clientmgr;

#[tauri::command]
pub fn cmd_virt_check_connection(host : &str) -> (usize,String)
{
    return clientmgr::virt_check_connection(host);
}