use hmir_errno::errno;
use hmir_ws_client_mgr::login::login_mgr;
use hmir_ws_client_mgr::ws_client_mgr;

//use log4rs;
use log::{error,info};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::Manager;
use tauri::WindowBuilder;

use hmir_ws_client_mgr::svr::svr_mgr as clientmgr;

#[tauri::command]
pub fn cmd_service_enabled(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_enabled_service(host);
}

#[tauri::command]
pub fn cmd_service_disabled(host : &str) -> (usize,String) {
    return clientmgr::svr_list_disabled_service(host);
}

#[tauri::command]
pub fn cmd_service_static(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_static_service(host);
}


#[tauri::command]
pub fn cmd_timer_enabled(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_enabled_timer(host);
}

#[tauri::command]
pub fn cmd_timer_disabled(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_disabled_timer(host);
}

#[tauri::command]
pub fn cmd_timer_static(host : &str) -> (usize,String)
{
    return clientmgr::svr_list_static_timer(host);
}