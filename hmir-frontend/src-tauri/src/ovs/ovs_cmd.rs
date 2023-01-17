use hmir_errno::errno;
use hmir_ws_client_mgr::login::login_mgr;
use hmir_ws_client_mgr::ws_client_mgr;

//use log4rs;
use log::{error,info};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::Manager;
use tauri::WindowBuilder;


use hmir_ws_client_mgr::net::net_mgr as clientmgr;


#[tauri::command]
fn greet(name : & str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

///返回后端进程信息，默认返回json字符串。
#[tauri::command]
fn cmd_process_info(host : & str) -> String {
    todo!();
}



#[tauri::command]
pub fn cmd_ovs_query_connection(host : &str) -> (usize,String)
{
    return clientmgr::ovs_query_connection(host);
}

#[tauri::command]
pub fn cmd_ovs_query_bridges(host : &str) -> (usize,String)
{
    return clientmgr::ovs_query_bridges(host);
}

#[tauri::command]
pub fn cmd_ovs_query_ports(host: &str) -> (usize, String)
{
    return clientmgr::ovs_query_ports(host);
}

#[tauri::command]
pub fn cmd_ovs_vsctl_add_br(host: &str, br_name: &str) ->(usize, String)
{
    return clientmgr::ovs_vsctl_add_br(host, br_name);
}

#[tauri::command]
pub fn cmd_ovs_vsctl_del_br(host: &str, br_name: &str) ->(usize, String)
{
    return clientmgr::ovs_vsctl_del_br(host, br_name);
}

#[tauri::command]
pub fn cmd_ovs_ofctl_forbid_dstip(host: &str, br_name:&str, dst_ip:&str, in_port:&str) -> (usize, String)
{
    return clientmgr::ovs_ofctl_forbid_dstip(host, br_name, dst_ip, in_port);
}