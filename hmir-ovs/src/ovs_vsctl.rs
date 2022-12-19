//! ovs-vsctl实现
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
//! - ovs-vsctl-set-netflow-rule： 网桥中设置netflow 规则
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-set-netflow-rule",
//!     "params": {"br_name":"ovsmgmt", "targets": "172.30.24.3:2055"} 
//! }
//!  ```
//! - ovs-vsctl-del-netflow-rule： 网桥中删除netflow 规则
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-del-netflow-rule",
//!     "params": {"br_name":"ovsmgmt"} 
//! }
//!  ```
//! - ovs-vsctl-set-port-vlan： 设置ovs port vlanID
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"vs-vsctl-set-port-vlan",
//!     "params": {"port_name":"vnet0", "tag_value":"2"} 
//! }
//!  ```

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

    module.register_method("ovs-vsctl-set-netflow-rule", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl_set_netflow_rule(br_info))
    })?;

    module.register_method("ovs-vsctl-del-netflow-rule", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl_del_netflow_rule(br_info))
    })?;

    module.register_method("ovs-vsctl-set-port-vlan", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        let port_name = br_info.get("port_name").unwrap();
        let tag_value =  br_info.get("tag_name").unwrap();
        Ok(ovs_vsctl_set_db_entry("Port".to_string(), port_name.to_string(), "tag".to_string(), tag_value.to_string()))
    })?;

    Ok(())
}


fn add_br(info_map : HashMap<String, String>) -> String {
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

fn del_br(info_map : HashMap<String, String>) -> String {
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

fn add_port(info_map : HashMap<String, String>) -> String {
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

fn del_port(info_map : HashMap<String, String>) -> String {
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

fn ovs_vsctl_set_netflow_rule(info_map : HashMap<String, String>) -> std::string::String {
    let br_name = info_map.get("br_name").unwrap();
    let targets =  info_map.get("targets").unwrap();
    let flow_rule = format!("netflow=@nf -- --id=@nf create NetFlow targets=\"{}\" active-timeout=60", targets);

    let output = Command::new(VSCTL_CMD)
                                        .arg("set")
                                        .arg("Bridge")
                                        .arg(br_name)
                                        .arg(flow_rule).
                                        output().expect("failed to excute ovs-vsctl-set-netflow-rule");
    if output.status.success() {
        String::from("Done")
    }else {
        String::from_utf8_lossy(&output.stderr).to_string()
    } 
}

fn ovs_vsctl_del_netflow_rule(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let output = Command::new(VSCTL_CMD)
                                        .arg("clear")
                                        .arg("Bridge")
                                        .arg(br_name)
                                        .arg("netflow").
                                        output().expect("failed to excute ovs-vsctl-del-netflow-rule");
    if output.status.success() {
        String::from("Done")
    }else {
        String::from_utf8_lossy(&output.stderr).to_string()
    } 
}

fn ovs_vsctl_set_db_entry(table: String, row: String, key: String, value: String) -> String{

    let set_rule = format!("{}={}", key, value);
    let output = Command::new(VSCTL_CMD)
                                        .arg("set")
                                        .arg(table)
                                        .arg(row)
                                        .arg(set_rule).
                                        output().expect("failed to excute ovs_vsctl_set_db_entry");
    if output.status.success() {
        String::from("Done")
    }else {
        String::from_utf8_lossy(&output.stderr).to_string()
    } 
}
