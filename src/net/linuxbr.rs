//! linux bridge 网桥能力
//! 网桥管理命令 brctl
//! 
//! 支持以下的格式：
//! - brctl-add-br: 增加linux网桥
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"brctl-add-br" ,
//!     "params": {"br_name":"ovirtmgmt"}
//! }
//! 响应格式：
//! {
//!     "jsonrpc":"2.0",
//!     "result":"Done",
//!     "id":1
//! }
//! 
//! - brctl-del-br: 删除linux网桥
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"brctl-del-br" ,
//!     "params": {"br_name":"ovirtmgmt"}
//! }
//! 
//! - brctl-add-interface: linux网桥中增加网络接口
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"brctl-add-br" ,
//!     "params": {"br_name":"ovirtmgmt", "port_name":"ens3"}
//! }
//! 
//! - brctl-del-interface: linux网桥中删除网络接口
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"brctl-del-interface" ,
//!     "params": {"br_name":"ovirtmgmt", "port_name":"ens3"}
//! }
//! 
//! - brctl-stp-on: linux网桥开启stp
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"brctl-stp-on" ,
//!     "params": {"br_name":"ovirtmgmt"}
//! }
//! 
//! - brctl-stp-off: linux网桥关闭stp
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"brctl-stp-off" ,
//!     "params": {"br_name":"ovirtmgmt"}
//! }
//! 

use jsonrpsee::ws_server::RpcModule;
use std::collections::HashMap;

use std::process::{Output};
use std::process::Command;

const BRCTL_CMD: &str = "/usr/sbin/brctl";

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("brctl-add-br", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_add_br(br_info))
    })?;

    module.register_method("brctl-del-br", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_del_br(br_info))
    })?;

    module.register_method("brctl-add-interface", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_add_interface(br_info))
    })?;

    module.register_method("brctl-del-interface", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_del_interface(br_info))
    })?;

    module.register_method("brctl-stp-on", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_stp_on(br_info))
    })?;

    module.register_method("brctl-stp-off", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_stp_off(br_info))
    })?;

    Ok(())
}


fn brctl_add_br(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} addbr {}", BRCTL_CMD, br_name);
    
    let output = exec_rule(rule, "brctl_add_br".to_string());
    reflect_cmd_result(output)
}

fn brctl_del_br(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} delbr {}", BRCTL_CMD, br_name);
    
    let output = exec_rule(rule, "brctl_del_br".to_string());
    reflect_cmd_result(output)
}

fn brctl_add_interface(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let port_name = info_map.get("port_name").unwrap();
    let rule = format!("{} addif {} {}", BRCTL_CMD, br_name, port_name);

    let output = exec_rule(rule, "brctl_add_interface".to_string());
    reflect_cmd_result(output)

}

fn brctl_del_interface(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let port_name = info_map.get("port_name").unwrap();
    let rule = format!("{} delif {} {}", BRCTL_CMD, br_name, port_name);

    let output = exec_rule(rule, "brctl_del_interface".to_string());
    reflect_cmd_result(output)
}

fn brctl_stp_on(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} stp {} on", BRCTL_CMD, br_name);
    
    let output = exec_rule(rule, "brctl_del_br".to_string());
    reflect_cmd_result(output)
}

fn brctl_stp_off(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} stp {} off", BRCTL_CMD, br_name);
    
    let output = exec_rule(rule, "brctl_del_br".to_string());
    reflect_cmd_result(output)
}

fn reflect_cmd_result(output : Output) -> String{

    if output.status.success(){
        String::from("Done")
    } else {
        String::from_utf8_lossy(&output.stderr).to_string()
    }
}

fn exec_rule(rule: String, cmd_name: String) -> Output{
    let output = Command::new("sh")
                        .arg("-c")
                        .arg(rule).
                        output().expect(&format!("failed to excute {}", cmd_name));

    output 
}