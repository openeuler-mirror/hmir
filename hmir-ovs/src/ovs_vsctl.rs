//! ovs-vsctl实现，网桥操作为主
//! 
//! 支持以下的格式
//! - ovs-vsctl-add-br: 添加网桥
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-add-br" ,
//!     "params": {"br_name":"ovsmgmt"}
//! }
//!  ```
//! 响应格式：
//! ```
//! {
//!     "jsonrpc":"2.0",
//!     "result":"Done",
//!     "id":1
//! }
//! ```
//! 
//! - ovs-vsctl-del-br： 删除网桥
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-del-br",
//!     "params": {"br_name":"ovsmgmt"} 
//! }
//!  ```
//!
//! - ovs-vsctl-add-port： 网桥中添加端口
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-add-port",
//!     "params": {"br_name":"ovsmgmt", "port_name":"ens4"} 
//! }
//!  ```
//! 
//! - ovs-vsctl-del-port： 网桥中删除端口
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-del-port",
//!     "params": {"br_name":"ovsmgmt", "port_name": "ens4"} 
//! }
//!  ```
//! 


use std::process::Command;
use std::collections::HashMap;
use jsonrpsee::ws_server::RpcModule;

const VSCTL_CMD: &str= "ovs-vsctl";

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("ovs-vsctl-add-br", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(add_br(br_info))
    })?;

    module.register_method("ovs-vsctl-del-br", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(del_br(br_info))
    })?;

    module.register_method("ovs-vsctl-add-port", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(add_port(br_info))
    })?;

    module.register_method("ovs-vsctl-del-port", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(del_port(br_info))
    })?;

    Ok(())
}


fn add_br(info_map : HashMap<String, String>) -> std::string::String {
    let br_name = info_map.get("br_name").unwrap();
    let output = Command::new(VSCTL_CMD)
                                        .arg("add-br")
                                        .arg(br_name).
                                        output().expect("failed to excute ovs-vsctl-add-br");
    if output.status.success() {
        String::from("Done")
    }else {
        String::from_utf8_lossy(&output.stderr).to_string()
    } 
}

fn del_br(info_map : HashMap<String, String>) -> std::string::String {
    let br_name = info_map.get(VSCTL_CMD).unwrap();
    let output = Command::new("ovs-vsctl")
                                        .arg("del-br")
                                        .arg(br_name).
                                        output().expect("failed to excute ovs-vsctl-del-br");
    if output.status.success() {
        String::from("Done")
    }else {
        String::from_utf8_lossy(&output.stderr).to_string()
    }    
}

fn add_port(info_map : HashMap<String, String>) -> std::string::String {
    let br_name = info_map.get("br_name").unwrap();
    let port_name = info_map.get("port_name").unwrap();
    let output = Command::new(VSCTL_CMD)
                                        .arg("add-port")
                                        .arg(br_name)
                                        .arg(port_name).
                                        output().expect("failed to excute ovs-vsctl-add-port");
    if output.status.success() {
        String::from("Done")
    }else {
        String::from_utf8_lossy(&output.stderr).to_string()
    } 
}

fn del_port(info_map : HashMap<String, String>) -> std::string::String {
    let br_name = info_map.get("br_name").unwrap();
    let port_name = info_map.get("port_name").unwrap();
    let output = Command::new(VSCTL_CMD)
                                        .arg("del-port")
                                        .arg(br_name)
                                        .arg(port_name).
                                        output().expect("failed to excute ovs-vsctl-del-port");
    if output.status.success() {
        String::from("Done")
    }else {
        String::from_utf8_lossy(&output.stderr).to_string()
    } 
}
