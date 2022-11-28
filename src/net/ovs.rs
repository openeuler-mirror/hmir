//! ovs管理模块
//! 
//! 支持以下的格式
//! - ovs-check-connection: 查询ovsdb连接状态
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-check-connection" 
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
//!     "method":"ovs-get-bridges" 
//! }
//!  ```
//! 响应格式：
//! ```
//! {
//!     "jsonrpc":"2.0",
//!     "result":"{\"ovs_bridges\":[{\"name\":\"br-br0\",\"uuid\":\"d9879f03-fa8d-49ee-8905-61bf2e678a94\",\"ports\":[{\"name\":\"ens3\",\"uuid\":\"29bb1048-53f2-41c8-8d76-2592045312c9\",\"mode\":{\"Trunk\":[]}},{\"name\":\"br-br0\",\"uuid\":\"7a3c43b0-7cb6-47bb-9e73-b7acfbce3d78\",\"mode\":{\"Trunk\":[]}},{\"name\":\"patch-out\",\"uuid\":\"c863f5e9-e312-4694-9279-01650284d3ae\",\"mode\":{\"Trunk\":[]}}]},{\"name\":\"br-test\",\"uuid\":\"fbc4b9ea-930b-4a53-845a-68d9bf5d46e4\",\"ports\":[{\"name\":\"patch-in\",\"uuid\":\"1cc48e17-63b7-4af3-a2d2-d709f9152497\",\"mode\":{\"Trunk\":[]}},{\"name\":\"vnet1\",\"uuid\":\"715a810a-c3e4-4407-86d7-c003add84406\",\"mode\":{\"Access\":100}},{\"name\":\"br-test\",\"uuid\":\"c082c812-3a84-4104-a8d3-c93720cbd959\",\"mode\":{\"Trunk\":[]}}]}]}",
//!    "id":1
//! }
//! ```
//!  //! - ovs-get-ports： 查询ovs端口信息
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-get-ports" 
//! }
//!  ```
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



use std::collections::HashMap;
use serde_json::json;
use std::fmt;
use std::error::Error;
use std::fmt::Display;

use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::net::Shutdown;
use std::string::FromUtf8Error;
use uuid::Uuid;
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

fn u8v_to_string(v : Vec<u8>) -> Result<String, FromUtf8Error>{
    String::from_utf8(v)
}

// ovs bridge definiton
#[derive(Debug, Clone,Serialize)]
pub struct OvsBridge{
    pub name : String,
    pub uuid : String,
    pub ports : Vec<OvsPort>
}

impl OvsBridge{
    pub fn new(name:&str, uuid:&str) -> OvsBridge{
        OvsBridge{
            name: name.to_string(),
            uuid : uuid.to_string(),
            ports : Vec::new()
        }
    }
}

// ovs port definition
#[derive(Debug, Clone, Serialize)]
pub struct OvsPort{
    pub name : String,
    pub uuid : String,
    pub mode : OvsPortMode
}

#[derive(Debug, Clone, Serialize)]
pub enum OvsPortMode{
    Access(u16),
    Trunk(Vec<u16>)
}

impl OvsPort{
    pub fn new(name:&str, uuid:&str, mode:&OvsPortMode) -> OvsPort{
        OvsPort{
            name: name.to_string(),
            uuid : uuid.to_string(),
            mode : mode.clone()
        }
    }
}

// ovs error definition
#[derive(Debug)]
pub struct OvsError{
    pub error_type: OvsErrorType,
    pub error_message: String,
    pub error_detail: String
}

#[derive(Debug)]
pub enum OvsErrorType{
    ConnectionError,
    InvalidResponse,
    InvalidResponseJson,
    UnexpectedResponse,
    InconsistentInstruction,
    QueryError
}

impl OvsError{
    pub fn new(t: OvsErrorType, message: &str) -> OvsError{
        OvsError{
            error_type: t,
            error_message : message.to_string(),
            error_detail : "".to_string()
        }
    }

    pub fn detail(mut self, detail: &str) -> Self{
        self.error_detail = detail.to_string();
        self
    }
}

impl Error for OvsError{
    fn description(&self) -> &str{
        "OvsError"
    }
}

impl Display for OvsError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        if self.error_detail == "" {
            write!(f, "[OvsError]{}", self.error_message)
        }
        else{
            write!(f, "[OvsError]{}\n  ->(detail){}", self.error_message, self.error_detail)
        }
    }
}

// ovs interface
pub struct OvsClient{
    transaction_id : i32,
    target : String
}

impl OvsClient{
    pub fn new() -> Result<OvsClient, OvsError>{
        Ok(OvsClient{
            transaction_id : 0,
            target : String::from("/var/run/openvswitch/db.sock")
        })
    }

    fn check_connection(&mut self) -> bool{
        let query = serde_json::from_str(
            "{\"method\": \"transact\",\"params\":[\"Open_vSwitch\",{\"op\":\"select\",\"table\":\"Port\",\"where\":[]}],\"id\":0}"
        ).unwrap();
        
        match self.send_msg(query){
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn get_ports(&mut self) -> Result<Vec<OvsPort>, OvsError>{
        let query = serde_json::from_str(
            "{\"method\": \"transact\",\"params\":[\"Open_vSwitch\",{\"op\":\"select\",\"table\":\"Port\",\"where\":[]}],\"id\":0}"
        ).unwrap();
        
        let resp = self.send_msg(query);
        let mut ports:Vec<OvsPort> = Vec::new();
        
        match resp{
            Err(e) => return Err(e),
            Ok(data) => {
                for p in data["result"][0]["rows"].as_array().unwrap(){
                    
                    let name: &str= p["name"].as_str().ok_or(
                        OvsError::new(
                            OvsErrorType::UnexpectedResponse,
                            "key ['name'] is not found in response data"
                        )
                    )?;
                    
                    let uuid: &str= p["_uuid"][1].as_str().ok_or(
                        OvsError::new(
                            OvsErrorType::UnexpectedResponse,
                            "key [_'uuid'][1] is not found in response data"
                        )
                    )?;
                    
                    if p["trunks"][1].as_array().unwrap().len() > 0{
                        let mut trunk_vlans : Vec<u16> = Vec::new();
                        for vlan in p["trunks"][1].as_array().unwrap(){
                            trunk_vlans.push(vlan.as_u64().unwrap() as u16);
                        }
                        
                        ports.push(OvsPort::new(
                            name,
                            uuid,
                            &OvsPortMode::Trunk(trunk_vlans.clone())
                        ));
                        continue;
                    }
                    
                    match p["tag"].as_u64(){
                        None => {},
                        Some(vlan) => {
                            ports.push(OvsPort::new(
                                name,
                                uuid,
                                &OvsPortMode::Access(vlan as u16)
                            ));
                            continue;
                        }
                    }
                    
                    ports.push(OvsPort::new(
                        name,
                        uuid,
                        &OvsPortMode::Trunk(Vec::new())
                    ));
                    
                }
            }
        }
        
        Ok(ports)
    }

    fn get_bridges(&mut self) -> Result<Vec<OvsBridge>, OvsError>{
        let query = serde_json::from_str(
            "{\"method\": \"transact\",\"params\":[\"Open_vSwitch\",{\"op\":\"select\",\"table\":\"Bridge\",\"where\":[]}],\"id\":0}"
        ).unwrap();
        
        let resp = self.send_msg(query);
        let mut bridges:Vec<OvsBridge> = Vec::new();
        
        match resp{
            Err(e) => return Err(e),
            Ok(data) => {
                let ports = self.get_ports()?;
                
                for br in data["result"][0]["rows"].as_array().unwrap(){
                    let name: &str= br["name"].as_str().ok_or(
                        OvsError::new(
                            OvsErrorType::UnexpectedResponse,
                            "key ['name'] is not found in response data"
                        )
                    )?;
                    
                    let uuid: &str= br["_uuid"][1].as_str().ok_or(
                        OvsError::new(
                            OvsErrorType::UnexpectedResponse,
                            "key [_'uuid'][1] is not found in response data"
                        )
                    )?;
                    
                    let mut b = OvsBridge::new(
                        name,
                        uuid
                    );
                    
                    match br["ports"][1].as_array(){
                        None=>{},
                        Some(br_ports)=>{
                            for bp in br_ports{
                                for p in &ports{
                                    if bp[1]== p.uuid{
                                        b.ports.push(p.clone());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    
                    bridges.push(b);
                }
            }
        }
        
        Ok(bridges)
    }

    fn add_port(&mut self, bridge_name:&str, port_name: &str, port_mode: &OvsPortMode) -> Result<serde_json::Value, OvsError>{
        let ports = self.get_ports()?;
        let bridges = self.get_bridges()?;
        
        for p in ports{
            if p.name == port_name{
                return Err(
                        OvsError::new(
                        OvsErrorType::InconsistentInstruction,
                        &format!("Interface already exist in ovsdb. ({})", port_name)
                    )
                )
            }
        }
        
        //let mut target_bridge: Option<&OvsBridge> = None;
        //let target_bridge = self.get_bridge(bridge_name);
        let mut port_list:Vec<Vec<String>> = Vec::new();
        
        let target_bridge = match self.get_bridge(bridge_name){
            None=>{
                return Err(
                        OvsError::new(
                        OvsErrorType::InconsistentInstruction,
                        &format!("Bridge is not found. ({})", bridge_name)
                    )
                )
            },
            Some(b) =>{
                println!("{}", serde_json::to_string(&b).unwrap());
                for p in &b.ports{
                    port_list.push(vec!("uuid".to_string(), p.uuid.clone()));
                }
                println!("{}", serde_json::to_string(&port_list).unwrap());
                b
            }
        };
        
        let interface_tmp_uuid = format!("row{}", Uuid::new_v4()).replace("-", "_");
        let port_tmp_uuid = format!("row{}", Uuid::new_v4()).replace("-", "_");
        
        port_list.push(vec!("named-uuid".to_string(), port_tmp_uuid.clone()));
        
        let mut row_json = json!({});
        match port_mode{
            &OvsPortMode::Access(vlan)=>{
                row_json = json!({
                    "name" : port_name,
                    "interfaces":[
                        "named-uuid",
                        interface_tmp_uuid
                    ],
                    "tag" : vlan
                });
            },
            &OvsPortMode::Trunk(ref vlans)=>{
                let mut vlan_list:Vec<u64> = Vec::new();
                for vlan in vlans{
                    vlan_list.push(vlan.clone().into());
                }
                let trunks = json!(["set",vlan_list]);
                row_json = json!({
                    "name" : port_name,
                    "interfaces":[
                        "named-uuid",
                        interface_tmp_uuid
                    ],
                    "trunks" : trunks
                });
            }
        }
        
        let mut query = json!({
            "method":"transact",
            "params":[
                "Open_vSwitch",
                {
                    "uuid-name" : interface_tmp_uuid,
                    "op" : "insert",
                    "table" : "Interface",
                    "row": {
                        "name":port_name,
                        "type":""
                    }
                },
                {
                    "uuid-name": port_tmp_uuid,
                    "op" : "insert",
                    "table" : "Port",
                    "row":row_json
                },
                {
                    "where": [
                        [
                            "_uuid",
                            "==",
                            [
                                "uuid",
                                target_bridge.uuid
                            ]
                        ]
                    ],
                    "row": {
                        "ports": [
                            "set",
                            port_list
                        ]
                    },
                    "op": "update",
                    "table": "Bridge"
                }
            ],
            "id":self.transaction_id
        });
        
        println!("query_message:{}", query);
        self.send_msg(query)
    }

    fn get_bridge(&mut self, bridge_name:&str) -> Option<OvsBridge>{
        let bridges = self.get_bridges().unwrap();
        
        for i in  0..bridges.len(){
            if bridges[i].name == bridge_name{
                return Some(bridges[i].clone());
            }
        }
        
        None
    }

    fn send_msg(&mut self, msg : serde_json::Value) -> Result<serde_json::Value, OvsError>{
        self.transaction_id += 1;
        let mut socket = match UnixStream::connect(&self.target){
            Ok(con) => con,
            Err(e) =>return  Err(
                OvsError::new(
                    OvsErrorType::ConnectionError,
                    &format!("failed to connect ovs ({})", self.target)
                )
                .detail(&e.to_string())
            )
        };
        
        //let mut socket = UnixStream::connect(&self.target)?;     
        socket.write_all(msg.to_string().as_bytes())
        .map_err(
            |e| OvsError::new(OvsErrorType::ConnectionError, "Faild to send request data").detail(&e.to_string())
        )?;
        
        let _ = socket.flush();
        let _ = socket.shutdown(Shutdown::Write);
        
        let mut s : Vec<u8> = Vec::new();
        let resp_str = (
            socket.read_to_end(&mut s)
            .map_err(
                |_| OvsError::new(OvsErrorType::InvalidResponse, "Failed to read response data")
            )
            .and_then(
                |_| u8v_to_string(s)
                .map_err(
                    |_| OvsError::new(OvsErrorType::InvalidResponse, "Failed to read response data")
                )
            )
        )?;

        let resp_json: serde_json::Value = (
            serde_json::from_str(resp_str.as_str())
            .map_err(
                |_| OvsError::new(OvsErrorType::InvalidResponseJson, "Faild to parse response data")
            )
        )?;

        match resp_json["result"][0].as_object(){
            None => {
                return Err(OvsError::new(OvsErrorType::InvalidResponse, "Faild to parse response data"))
            },
            Some(result) => {
                if result.contains_key("error"){
                    return Err(
                        OvsError::new(OvsErrorType::QueryError, "Client received error response. Please check detail.")
                        .detail(&resp_json["result"][0].to_string())
                    )
                }
            }
        }

        Ok(resp_json)
    }
    
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
