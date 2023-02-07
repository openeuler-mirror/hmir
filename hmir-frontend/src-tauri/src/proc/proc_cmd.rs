use hmir_errno::errno;
use hmir_ws_client_mgr::login::login_mgr;
use hmir_ws_client_mgr::ws_client_mgr;

//use log4rs;
use log::{error,info};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::Manager;
use tauri::WindowBuilder;

use hmir_ws_client_mgr::proc::proc_mgr as clientmgr;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn cmd_process_info(host : & str) -> (usize,String) {
    return clientmgr::proc_process_info(host);
}

