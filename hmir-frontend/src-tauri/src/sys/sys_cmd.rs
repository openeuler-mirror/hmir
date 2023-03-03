

use hmir_ws_client_mgr::sys::sys_mgr as clientmgr;

#[tauri::command]
pub fn cmd_sys_info(host : &str) -> (usize,String)
{
    return clientmgr::sys_os_all_info(host);
}

#[tauri::command]
pub fn cmd_sys_pci_info(host : &str) -> (usize,String)
{
    return clientmgr::sys_list_pci_info(host);
}

#[tauri::command]
pub fn cmd_sys_get_date(host : &str) ->  (usize,String) {
    return clientmgr::sys_get_date(host);
}

#[tauri::command]
pub fn cmd_sys_set_date(host : &str, date : String) -> (usize,String)
{
    return clientmgr::sys_set_date(host,date);
}

#[tauri::command]
pub fn cmd_sys_set_hostname(host: & str, pretty_name : String,static_name : String) -> (usize,String)
{
    return clientmgr::sys_set_hostname(host,pretty_name,static_name);
}