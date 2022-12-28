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
fn cmd_logout(host : &str)
{
    return clientmgr::logout(host);
}


fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,
            cmd_login,
            cmd_logout,
            cmd_ttyd_start])
        // .invoke_handler(tauri::generate_handler![ttyd_start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
