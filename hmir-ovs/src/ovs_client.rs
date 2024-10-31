use super::ovs_error::*;
use super::ovs_port::*;
use super::ovs_bridge::*;

use serde_json::json;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::net::Shutdown;
use std::string::FromUtf8Error;
use uuid::Uuid;


fn u8v_to_string(v : Vec<u8>) -> Result<String, FromUtf8Error>{
    String::from_utf8(v)
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

    pub fn check_connection(&mut self) -> bool{
        let query = serde_json::from_str(
            "{\"method\": \"transact\",\"params\":[\"Open_vSwitch\",{\"op\":\"select\",\"table\":\"Port\",\"where\":[]}],\"id\":0}"
        ).unwrap();
        
        match self.send_msg(query){
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn get_ports(&mut self) -> Result<Vec<OvsPort>, OvsError>{
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

    pub fn get_interfaces(&mut self) -> Result<Vec<OvsInterface>, OvsError>{

        let query = serde_json::from_str(
            "{\"method\": \"transact\",\"params\":[\"Open_vSwitch\",{\"op\":\"select\",\"table\":\"Interface\",\"where\":[]}],\"id\":0}"
        ).unwrap();

        let resp = self.send_msg(query);
        let mut interfaces:Vec<OvsInterface> = Vec::new();

        match resp{
            Err(e) => return Err(e),
            Ok(data) => {
                for p in data["result"][0]["rows"].as_array().unwrap()
                {
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

                    let mac: &str= p["mac_in_use"].as_str().ok_or(
                        OvsError::new(
                            OvsErrorType::UnexpectedResponse,
                            "key ['mac_in_use'] is not found in response data"
                        )
                    )?;
                    interfaces.push(OvsInterface::new(
                        name,
                        uuid,
                        mac
                    ));
                }
            }
        }
        Ok(interfaces)
    }

    pub fn get_netflows(&mut self) -> Result<OvsNetflow, OvsError>{
        let query = serde_json::from_str(
            "{\"method\": \"transact\",\"params\":[\"Open_vSwitch\",{\"op\":\"select\",\"table\":\"NetFlow\",\"where\":[]}],\"id\":0}"
        ).unwrap();

        let resp = self.send_msg(query);
        let mut netflow = OvsNetflow::new("", "");

        match resp{
            Err(e) => return Err(e),
            Ok(data) => {
                for p in data["result"][0]["rows"].as_array().unwrap()
                {
                    let uuid: &str= p["_uuid"][1].as_str().ok_or(
                        OvsError::new(
                            OvsErrorType::UnexpectedResponse,
                            "key [_'uuid'][1] is not found in response data"
                        )
                    )?;

                    let targets: &str= p["targets"].as_str().ok_or(
                        OvsError::new(
                            OvsErrorType::UnexpectedResponse,
                            "key ['targets'] is not found in response data"
                        )
                    )?;
                    netflow = OvsNetflow::new(uuid, targets);
                }
            }
        }
        Ok(netflow)

    }

    pub fn get_ipfix(&mut self) -> Result<OvsIpfix, OvsError>{
        let query = serde_json::from_str(
            "{\"method\": \"transact\",\"params\":[\"Open_vSwitch\",{\"op\":\"select\",\"table\":\"IPFIX\",\"where\":[]}],\"id\":0}"
        ).unwrap();

        let resp = self.send_msg(query);
        let mut ipfix = OvsIpfix::new("", "");

        match resp{
            Err(e) => return Err(e),
            Ok(data) => {
                for p in data["result"][0]["rows"].as_array().unwrap()
                {
                    let uuid: &str= p["_uuid"][1].as_str().ok_or(
                        OvsError::new(
                            OvsErrorType::UnexpectedResponse,
                            "key [_'uuid'][1] is not found in response data"
                        )
                    )?;

                    let targets: &str= p["targets"].as_str().ok_or(
                        OvsError::new(
                            OvsErrorType::UnexpectedResponse,
                            "key ['targets'] is not found in response data"
                        )
                    )?;
                    ipfix = OvsIpfix::new(uuid, targets);
                }
            }
        }
        Ok(ipfix)

    }

    pub fn get_bridges(&mut self) -> Result<Vec<OvsBridge>, OvsError>{
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

    pub fn add_port(&mut self, bridge_name:&str, port_name: &str, port_mode: &OvsPortMode) -> Result<serde_json::Value, OvsError>{
        let ports = self.get_ports()?;
        
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
        
        let mut _row_json = json!({});
        match port_mode{
            &OvsPortMode::Access(vlan)=>{
                _row_json = json!({
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
                _row_json = json!({
                    "name" : port_name,
                    "interfaces":[
                        "named-uuid",
                        interface_tmp_uuid
                    ],
                    "trunks" : trunks
                });
            }
        }
        
        let query = json!({
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
                    "row":_row_json
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

    pub fn del_port(&mut self, bridge_name:&str, port_name: &str) -> Result<serde_json::Value, OvsError>{
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
                    if p.name != port_name {
                        port_list.push(vec!("uuid".to_string(), p.uuid.clone()));
                    }
                }
                println!("{}", serde_json::to_string(&port_list).unwrap());
                b
            }
        };

        let  query = json!({
            "method":"transact",
            "params":[
                "Open_vSwitch",
                {
                    "op" : "delete",
                    "table" : "Interface",
                    "where": [
                        [
                            "name",
                            "==",
                            port_name
                        ]
                    ]
                },
                {
                    "op" : "delete",
                    "table" : "Port",
                    "where": [
                        [
                            "name",
                            "==",
                            port_name   
                        ]
                    ]
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
                },
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
            |e| OvsError::new(OvsErrorType::ConnectionError, "Failed to send request data").detail(&e.to_string())
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
                |_| OvsError::new(OvsErrorType::InvalidResponseJson, "Failed to parse response data")
            )
        )?;

        match resp_json["result"][0].as_object(){
            None => {
                return Err(OvsError::new(OvsErrorType::InvalidResponse, "Failed to parse response data"))
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
