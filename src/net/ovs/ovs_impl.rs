
use super::ovs_bridge::*;
use super::ovs_port::*;
use super::ovs_client::*;

use std::collections::HashMap;
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


pub fn check_connection() -> std::string::String{
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

pub fn get_ports() -> std::string::String{
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