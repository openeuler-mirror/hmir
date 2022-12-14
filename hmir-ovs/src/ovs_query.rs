//! ovsdb RPC json实现，查询为主
//! 
//! 支持以下的格式
//! - ovs-check-connection: 查询ovsdb连接状态
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-query-connection" 
//! }
//!  ```
//! 响应格式：
//! ```
//! {
//!     "jsonrpc":"2.0",
//!     "result":"{\"message\":\"Done\"}",
//!     "id":1
//! }
//! ```
//! 
//! - ovs-get-bridges： 查询ovs网桥信息
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-query-bridges" 
//! }
//!  ```
//! 
//! 响应格式：
//! ```
//! {
//!     "jsonrpc":"2.0",
//!     "result":"{\"ovs_bridges\":[{\"name\":\"br-br0\",\"uuid\":\"d9879f03-fa8d-49ee-8905-61bf2e678a94\",\"ports\":[{\"name\":\"ens3\",\"uuid\":\"29bb1048-53f2-41c8-8d76-2592045312c9\",\"mode\":{\"Trunk\":[]}},{\"name\":\"br-br0\",\"uuid\":\"7a3c43b0-7cb6-47bb-9e73-b7acfbce3d78\",\"mode\":{\"Trunk\":[]}},{\"name\":\"patch-out\",\"uuid\":\"c863f5e9-e312-4694-9279-01650284d3ae\",\"mode\":{\"Trunk\":[]}}]},{\"name\":\"br-test\",\"uuid\":\"fbc4b9ea-930b-4a53-845a-68d9bf5d46e4\",\"ports\":[{\"name\":\"patch-in\",\"uuid\":\"1cc48e17-63b7-4af3-a2d2-d709f9152497\",\"mode\":{\"Trunk\":[]}},{\"name\":\"vnet1\",\"uuid\":\"715a810a-c3e4-4407-86d7-c003add84406\",\"mode\":{\"Access\":100}},{\"name\":\"br-test\",\"uuid\":\"c082c812-3a84-4104-a8d3-c93720cbd959\",\"mode\":{\"Trunk\":[]}}]}]}",
//!    "id":1
//! }
//! ```
//! 
//!  - ovs-get-ports： 查询ovs端口信息
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-query-ports" 
//! }
//!  ```
//! 
//! 响应格式：
//! ```
//! {
//!     "jsonrpc":"2.0",
//!     "result":"{\"ovs_ports\":[{\"name\":\"br-br0\",\"uuid\":\"7a3c43b0-7cb6-47bb-9e73-b7acfbce3d78\",\"mode\":{\"Trunk\":[]}},{\"name\":\"br-test\",\"uuid\":\"c082c812-3a84-4104-a8d3-c93720cbd959\",\"mode\":{\"Trunk\":[]}},{\"name\":\"ens3\",\"uuid\":\"29bb1048-53f2-41c8-8d76-2592045312c9\",\"mode\":{\"Trunk\":[]}},{\"name\":\"patch-in\",\"uuid\":\"1cc48e17-63b7-4af3-a2d2-d709f9152497\",\"mode\":{\"Trunk\":[]}},{\"name\":\"patch-out\",\"uuid\":\"c863f5e9-e312-4694-9279-01650284d3ae\",\"mode\":{\"Trunk\":[]}},{\"name\":\"vnet1\",\"uuid\":\"715a810a-c3e4-4407-86d7-c003add84406\",\"mode\":{\"Access\":100}}]}",
//!     "id":1}
//! ``` 
//! 
//! - ovs-add-port： ovs网桥中添加端口
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-add-port"
//!      "params":{"br_name":"br-br0","port_name":"ens4", "vlan_id":"100"}
//! }
//!  ```
//! 
//! 响应格式：
//! ```
//! {
//!     "jsonrpc":"2.0",
//!      "result":"{\"error\":null,\"id\":5,\"result\":[{\"uuid\":[\"uuid\",\"43980cb5-4625-4bda-a909-063557a5fbff\"]},{\"uuid\":[\"uuid\",\"52b526c8-0624-4e59-9ee5-31e03d533485\"]},{\"count\":1}]}",
//!     "id":1
//! }
//! ```
//! 
//! - ovs-del-port： ovs网桥中删除端口
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-del-port"
//!     "params":{"br_name":"br-br0","port_name":"ens4"}
//! }
//!  ```
//! 响应模式：
//! ```
//! {
//!     "jsonrpc":"2.0",
//!     "result":"{\"error\":null,\"id\":2,\"result\":[{\"count\":1},{\"count\":1},{\"count\":1}]}",
//!     "id":1,
//! }
//! ```

use jsonrpsee::ws_server::RpcModule;
use std::collections::HashMap;

use super::ovs_bridge::*;
use super::ovs_port::*;
use super::ovs_client::*;

use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct RetPort{
    pub ovs_ports: Vec<OvsPort>,
}

#[derive(Clone, Serialize)]
pub struct RetBridge{
    pub ovs_bridges: Vec<OvsBridge>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RetInfo {
    pub message: String,
}

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
        
    module.register_method("ovs-query-connection", |_, _| {
        Ok(check_connection())
    })?;
    
    module.register_method("ovs-query-ports", |_, _| {
        Ok(get_ports())
    })?;

    module.register_method("ovs-query-bridges", |_, _| {
        Ok(get_bridges())
    })?;

    module.register_method("ovs-add-port", |params, _| {
        let port_info = params.parse::<HashMap<String, String>>()?;
        Ok(add_port(port_info))
    })?;

    module.register_method("ovs-del-port", |params, _| {
        let port_info = params.parse::<HashMap<String, String>>()?;
        Ok(del_port(port_info))
    })?;
    Ok(())
}


fn check_connection() -> std::string::String{
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            let ret_info = RetInfo {
                message: e.error_detail.clone(),
            };
            let ret_message = serde_json::to_string(&ret_info).unwrap();
            ret_message
        },
        Ok(mut c) => {
            let is_connected = c.check_connection();
            if is_connected {
                let ret_info = RetInfo {
                    message: "Done".to_string(),
                };
                let ret_message = serde_json::to_string(&ret_info).unwrap();
                ret_message
            } else {
                let ret_info = RetInfo {
                    message: "Failure".to_string(),
                };
                let ret_message = serde_json::to_string(&ret_info).unwrap();
                ret_message
            }
        }      
    }    
}

fn get_ports() -> std::string::String{
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            let ret_info = RetInfo {
                message: e.error_detail.clone(),
            };
            let ret_message = serde_json::to_string(&ret_info).unwrap();
            ret_message
        },
        Ok(mut c)=>{
            let ports = c.get_ports();
            match ports{
                Ok(ports) =>{
                    println!("number of port : {0}", ports.len());
                    let ret_info = RetPort {
                        ovs_ports: ports,
                    };
                    let ret_message = serde_json::to_string(&ret_info).unwrap();
                    ret_message    
                },
                Err(e) => {
                    let ret_info = RetInfo {
                        message: e.error_detail.clone(),
                    };
                    let ret_message = serde_json::to_string(&ret_info).unwrap();
                    ret_message
                }
            }
        }
    }
}

pub fn get_bridges() -> std::string::String {
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            let ret_info = RetInfo {
                message: e.error_detail.clone(),
            };
            let ret_message = serde_json::to_string(&ret_info).unwrap();
            ret_message
        },
        Ok(mut c)=>{
            let bridges = c.get_bridges();
            match bridges{
                Ok(bridges) =>{
                    println!("number of bridge: {0}", bridges.len());
                    let ret_info = RetBridge {
                        ovs_bridges: bridges,
                    };
                    let ret_message = serde_json::to_string(&ret_info).unwrap();
                    ret_message    
                },
                Err(e) => {
                    let ret_info = RetInfo {
                        message: e.error_detail.clone(),
                    };
                    let ret_message = serde_json::to_string(&ret_info).unwrap();
                    ret_message
                }
            }
        }
    }
}

pub fn add_port(info_map : HashMap<String, String>) -> std::string::String {
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            let ret_info = RetInfo {
                message: e.error_detail.clone(),
            };
            let ret_message = serde_json::to_string(&ret_info).unwrap();
            ret_message
        },
        Ok(mut c)=>{
            let br_name = info_map.get("br_name").unwrap();
            let port_name = info_map.get("port_name").unwrap();
            let vlan_str = info_map.get("vlan_id").unwrap();
            let vlan_id = vlan_str.parse::<u16>().unwrap();
            println!("br_name:{},port_name:{},vlan_id:{}", br_name, port_name, vlan_id);
            let add_result = c.add_port(br_name, port_name, &OvsPortMode::Access(vlan_id));
            match add_result {
                Ok(add_result) =>{
                    let ret_message = json!(add_result).to_string();
                    ret_message 
                },
                Err(e) => {
                    let ret_info = RetInfo {
                        message: e.error_detail.clone(),
                    };
                    let ret_message = serde_json::to_string(&ret_info).unwrap();
                    ret_message
                }
            }
        }
    }
}

pub fn del_port(info_map : HashMap<String, String>) -> std::string::String {
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            let ret_info = RetInfo {
                message: e.error_detail.clone(),
            };
            let ret_message = serde_json::to_string(&ret_info).unwrap();
            ret_message
        },
        Ok(mut c)=>{
            let br_name = info_map.get("br_name").unwrap();
            let port_name = info_map.get("port_name").unwrap();
            println!("br_name:{},port_name:{}", br_name, port_name);
            let add_result = c.del_port(br_name, port_name);
            match add_result {
                Ok(add_result) =>{
                    let ret_message = json!(add_result).to_string();
                    ret_message 
                },
                Err(e) => {
                    let ret_info = RetInfo {
                        message: e.error_detail.clone(),
                    };
                    let ret_message = serde_json::to_string(&ret_info).unwrap();
                    ret_message
                }
            }
        }
    }
}