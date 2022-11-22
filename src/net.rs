//! 网络管理模块
//! ovs 管理功能：通过unix套接字调用ovsdb Json RPC接口管理ovs网桥，ovsdb Json RPC文档 RFC7047
//!

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::error::Error;
use std::fmt::Display;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::net::Shutdown;
use std::string::FromUtf8Error;

fn u8v_to_string(v : Vec<u8>) -> Result<String, FromUtf8Error>{
    String::from_utf8(v)
}
// ovs port definition
#[derive(Debug, Clone,Serialize)]
pub struct OvsPort{
    pub name : String,
    pub uuid : String,
    pub mode : OvsPortMode
}

#[derive(Debug, Clone,Serialize)]
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

#[derive(Debug)]
// ovs error definition
pub enum OvsErrorType{
    ConnectionError,
    InvalidResponse,
    InvalidResponseJson,
    QueryError
}

#[derive(Debug)]
pub struct OvsError{
    pub error_type: OvsErrorType,
    pub error_message: String,
    pub error_detail: String
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
    target : String
}

impl OvsClient{
    pub fn new() -> Result<OvsClient, OvsError>{
        Ok(OvsClient{
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
        todo!() 
    }

    fn get_bridges(&mut self) -> Result<Vec<OvsBridge>, OvsError>{
        todo!() 
    }

    fn add_port(&mut self, bridge_name:&str, port_name: &str, port_mode: &OvsPortMode) -> Result<serde_json::Value, OvsError>{
        todo!() 
    }

    fn send_msg(&mut self, msg : serde_json::Value) -> Result<serde_json::Value, OvsError>{
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

#[derive(Clone, Serialize, Deserialize)]
struct RetInfo {
    message: String,
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
                    let mut ret_message  =  "".to_string();
                    for port in ports{
                        //ret_message = serde_json::to_string(&port).unwrap();
                    }
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
    todo!()
}

pub fn add_port() -> std::string::String {
    todo!()
}

#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

   module.register_method("check-connection", |_, _| {
        Ok(check_connection())
    })?;

    module.register_method("get-ports", |_, _| {
        Ok(get_ports())
    })?;

    module.register_method("get-bridges", |_, _| {
        Ok(get_bridges())
    })?;

    module.register_method("add-port", |_, _| {
        Ok(add_port())
    })?;

    Ok(())
}