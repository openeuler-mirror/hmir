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

    // log_init();

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
            cmd_ttyd_start])
        // .invoke_handler(tauri::generate_handler![ttyd_start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
