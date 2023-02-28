use hmir_errno::errno;
use hmir_ws_client_mgr::login::login_mgr;
use hmir_ws_client_mgr::ws_client_mgr;

//use log4rs;
use log::{error,info};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::Manager;
use tauri::WindowBuilder;

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
pub fn cmd_sys_set_date(host : &str, date : String) -> (usize,String)
{
    return clientmgr::sys_set_date(host,date);
}

#[tauri::command]
pub fn cmd_sys_set_hostname(host: & str, pretty_name : String,static_name : String) -> (usize,String)
{
    return clientmgr::sys_set_hostname(host,pretty_name,static_name);
}