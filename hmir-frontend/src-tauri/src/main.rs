//! 给前端提供tauri-command，用于和tauri后端交互
//!
//!
//! 支持的tauri命令
//! - cmd_ttyd_start: 启动控制台
//! - cmd_ttyd_stop : 停止控制台
//! - cmd_login         : 登录HMIR后端
//! - cmd_logout        : 注销
//! - cmd_quit          : 退出系统
//! - cmd_ovs_query_connection : 查询是否与ovs数据库建立链接
//! - cmd_ovs_query_bridges:  查询系统ovs网桥相关信息
//! - cmd_ovs_query_ports: 查询系统ovs ports相关信息
//! - cmd_ovs_vsctl_add_br : 创建ovs网桥
//! - cmd_ovs_vsctl_del_br : 删除ovs网桥
//! - cmd_ovs_ofctl_forbid_dstip : 禁止虚拟机访问外部IP地址
//! - cmd_virt_check_connection : 检查与本地hypervisor连接是否正常
//! ```
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};
use anyhow;
use futures::executor::block_on;
use tokio;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::Manager;
use tauri::WindowBuilder;

//use log4rs;
use log::{error,info};
use std::process;
use hmir_errno::errno;


mod login;
mod ttyd;
mod svr;
mod ovs;
mod virt;

use crate::login::login_cmd::*;
use crate::ttyd::ttyd_cmd::*;
use crate::svr::svr_cmd::*;
use crate::ovs::ovs_cmd::*;
use crate::virt::virt_cmd::*;



#[tauri::command]
fn cmd_error_description(code : usize) -> String {
    return String::from(errno::HMIR_MSG[code]);
}



#[tauri::command]
fn cmd_quit() {
    std::process::exit(0);
}

fn log_init ()
{
    #[cfg(target_os = "linux")]
    {
        let log = log4rs::init_file("/etc/hmir/log4rs.yaml",Default::default());
        match log {
            Err(e) => {
                println!("Err for init log : {}",e);
                process::exit(1);
            }
            _ => ()
        }
    }

    #[cfg(target_os = "unix")]
    {
        let log = log4rs::init_file("~/.config/hmir/log4rs.yaml",Default::default());
        match log {
            Err(e) => {
                println!("Err for init log : {}",e);
                process::exit(1);
            }
            _ => ()
        }
    }

}

fn main() {

    log_init();

    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.

    const SPLASH_WAIT_TIME : u64 = 5;

    tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // initialize your app here instead of sleeping :)
                std::thread::sleep(std::time::Duration::from_secs(SPLASH_WAIT_TIME));
                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![cmd_login,
            cmd_logout,
            cmd_ttyd_stop,
            cmd_ttyd_start,
            cmd_quit,
            cmd_service_enabled,
            cmd_service_disabled,
            cmd_service_static,
            cmd_timer_enabled,
            cmd_timer_disabled,
            cmd_timer_static,
            cmd_socket_enabled,
            cmd_socket_disabled,
            cmd_socket_static,
            cmd_target_enabled,
            cmd_target_disabled,
            cmd_target_static,
            cmd_all_slice,
            cmd_ovs_query_connection,
            cmd_ovs_query_bridges,
            cmd_ovs_query_ports,
            cmd_ovs_vsctl_add_br,
            cmd_ovs_vsctl_del_br,
            cmd_ovs_ofctl_forbid_dstip,
            cmd_virt_check_connection])
        // .invoke_handler(tauri::generate_handler![ttyd_start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
