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


//use log4rs;
//use log::{error,info};
use std::process;


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
fn cmd_login(host : & str, port : i32 , username : & str, password : & str) -> bool
{
    if clientmgr::register_client(host,port) {
        return clientmgr::login(host,username,password);
    }
    return false;
}

#[tauri::command]
fn cmd_logout(host : &str) -> bool
{
    return clientmgr::logout(host);
}


//fn log_init ()
//{
//    let log = log4rs::init_file("/etc/hmir/log4rs.yaml",Default::default());
//    match log {
//        Err(e) => {
//            println!("Err for init log : {}",e);
//            process::exit(1);
//        }
//        _ => ()
//    }
//}

fn main() {

    log_init();

    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let submenu = Submenu::new("文件", Menu::new().add_item(quit));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
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
            cmd_ttyd_start])
        // .invoke_handler(tauri::generate_handler![ttyd_start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
