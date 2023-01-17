use hmir_errno::errno;
use hmir_ws_client_mgr::login::login_mgr;
use hmir_ws_client_mgr::ws_client_mgr;

//use log4rs;
use log::{error,info};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::Manager;
use tauri::WindowBuilder;

use hmir_ws_client_mgr::ttyd::ttyd_mgr as clientmgr;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn cmd_ttyd_start(host : & str) -> bool {
    return clientmgr::ttyd_start(host);
}

#[tauri::command]
pub fn cmd_ttyd_stop(host : & str) -> bool {
    return clientmgr::ttyd_stop(host);
}