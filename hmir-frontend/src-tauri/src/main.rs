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
use tauri_plugin_websocket;

//use log4rs;
use log::{error,info};
use std::process;
use hmir_errno::errno;


mod wsclient;
mod clientmgr;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn cmd_ttyd_start(host : & str) -> bool {
    return clientmgr::ttyd_start(host);
}

#[tauri::command]
fn cmd_ttyd_stop(host : & str) -> bool {
    return clientmgr::ttyd_stop(host);
}

#[tauri::command]
fn greet(name : & str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn cmd_error_description(code : usize) -> String {
    return String::from(errno::HMIR_MSG[code]);
}

#[tauri::command]
fn cmd_login(host : & str, port : i32 , username : & str, password : & str) -> usize
{
    const use_ssh_login : bool = false;
    if clientmgr::register_client(host,port) {
        if use_ssh_login {
            return clientmgr::ssh_login(host,username,password);
        }
        return clientmgr::login(host,username,password);
    } else {
        error!("Can't register clinet : {}:{}",host,port);
        return errno::HMIR_ERR_CONNECT_SERVER;
    }
}

///返回后端进程信息，默认返回json字符串。
#[tauri::command]
fn cmd_process_info(host : & str) -> String {
    todo!();
}

#[tauri::command]
fn cmd_logout(host : &str) -> bool
{
    return clientmgr::logout(host);
}


#[tauri::command]
fn cmd_service_enabled(host : &str) -> (usize,String)
{
    return clientmgr::service_list_enabled(host);
}

#[tauri::command]
fn cmd_service_disabled(host : &str) -> (usize,String)
{
    return clientmgr::service_list_disabled(host);
}

#[tauri::command]
fn cmd_ovs_query_connection(host : &str) -> (usize,String)
{
    return clientmgr::ovs_query_connection(host);
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
        .invoke_handler(tauri::generate_handler![greet,
            cmd_login,
            cmd_logout,
            cmd_ttyd_stop,
            cmd_ttyd_start,
            cmd_quit,
            cmd_service_enabled,
            cmd_service_disabled,
            cmd_ovs_query_connection])
        // .invoke_handler(tauri::generate_handler![ttyd_start])
        .plugin(tauri_plugin_websocket::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
