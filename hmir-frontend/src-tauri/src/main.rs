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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn ttyd_start(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
fn greet(name : & str) -> String {
    let client = wsclient::RequestClient::new("172.30.24.123:5898".to_string());
    client.ttyd_start();
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,ttyd_start])
        // .invoke_handler(tauri::generate_handler![ttyd_start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
