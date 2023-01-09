//! ovsdb RPC json实现，查询为主
//! 
//! 支持以下的格式
//! - ovs-check-connection: 查询ovsdb连接状态
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-query-connection" 
//! }
//! 响应格式：
//! {
//!     "jsonrpc":"2.0",
//!     "result":"{\"message\":\"Done\"}",
//!     "id":1
//! }
//! 
//! - ovs-query-bridges： 查询ovs网桥信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-query-bridges" 
//! }
//! 响应格式：
//! {
//!     "jsonrpc":"2.0",
//!     "result":"[{\"name\":\"br-br0\",\"uuid\":\"d9879f03-fa8d-49ee-8905-61bf2e678a94\",\"ports\":[{\"name\":\"ens3\",\"uuid\":\"29bb1048-53f2-41c8-8d76-2592045312c9\",\"mode\":{\"Trunk\":[]}},{\"name\":\"br-br0\",\"uuid\":\"7a3c43b0-7cb6-47bb-9e73-b7acfbce3d78\",\"mode\":{\"Trunk\":[]}},{\"name\":\"patch-out\",\"uuid\":\"c863f5e9-e312-4694-9279-01650284d3ae\",\"mode\":{\"Trunk\":[]}}]},{\"name\":\"br-test\",\"uuid\":\"fbc4b9ea-930b-4a53-845a-68d9bf5d46e4\",\"ports\":[{\"name\":\"patch-in\",\"uuid\":\"1cc48e17-63b7-4af3-a2d2-d709f9152497\",\"mode\":{\"Trunk\":[]}},{\"name\":\"vnet1\",\"uuid\":\"715a810a-c3e4-4407-86d7-c003add84406\",\"mode\":{\"Access\":100}},{\"name\":\"br-test\",\"uuid\":\"c082c812-3a84-4104-a8d3-c93720cbd959\",\"mode\":{\"Trunk\":[]}}]}]}",
//!    "id":1
//! }
//! 
//!  - ovs-query-ports： 查询ovs端口信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-query-ports" 
//! }
//! 响应格式：
//! {
//!     "jsonrpc":"2.0",
//!     "result":"[{\"name\":\"br-br0\",\"uuid\":\"7a3c43b0-7cb6-47bb-9e73-b7acfbce3d78\",\"mode\":{\"Trunk\":[]}},{\"name\":\"br-test\",\"uuid\":\"c082c812-3a84-4104-a8d3-c93720cbd959\",\"mode\":{\"Trunk\":[]}},{\"name\":\"ens3\",\"uuid\":\"29bb1048-53f2-41c8-8d76-2592045312c9\",\"mode\":{\"Trunk\":[]}},{\"name\":\"patch-in\",\"uuid\":\"1cc48e17-63b7-4af3-a2d2-d709f9152497\",\"mode\":{\"Trunk\":[]}},{\"name\":\"patch-out\",\"uuid\":\"c863f5e9-e312-4694-9279-01650284d3ae\",\"mode\":{\"Trunk\":[]}},{\"name\":\"vnet1\",\"uuid\":\"715a810a-c3e4-4407-86d7-c003add84406\",\"mode\":{\"Access\":100}}]}",
//!     "id":1}
//! }
//! 
//! - ovs-add-port： ovs网桥中添加端口
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-add-port"
//!      "params":{"br_name":"br-br0","port_name":"ens4", "vlan_id":"100"}
//! }
//! 响应格式：
//! {
//!     "jsonrpc":"2.0",
//!      "result":"{\"error\":null,\"id\":5,\"result\":[{\"uuid\":[\"uuid\",\"43980cb5-4625-4bda-a909-063557a5fbff\"]},{\"uuid\":[\"uuid\",\"52b526c8-0624-4e59-9ee5-31e03d533485\"]},{\"count\":1}]}",
//!     "id":1
//! }
//! 
//! - ovs-del-port： ovs网桥中删除端口
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-del-port"
//!     "params":{"br_name":"br-br0","port_name":"ens4"}
//! }
//! 响应模式：
//! {
//!     "jsonrpc":"2.0",
//!     "result":"{\"error\":null,\"id\":2,\"result\":[{\"count\":1},{\"count\":1},{\"count\":1}]}",
//!     "id":1,
//! }
//! 
//!  - ovs-query-interfaces： 查询ovs interface信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-query-interfaces" 
//! }
//! 响应格式：
//! {
//!     "jsonrpc":"2.0",
//!      "result":"[{\"name\":\"br-eth-patch\",\"uuid\":\"f01fe186-f028-4514-a8e8-655fd0c0574c\",\"mac\":\"32:37:9d:53:2d:fc\"},{\"name\":\"br-test-patch\",\"uuid\":\"4d3d9983-4d5b-4a8f-ac98-31b7979aeec7\",\"mac\":\"42:7f:e2:9d:c4:fe\"}]",
//!      "id":1
//! }
//! 
//!  - ovs-query-netflows： 查询ovs netflow信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-query-netflows" 
//! }
//! 响应格式：
//! {
//!     "jsonrpc":"2.0",
//!      "result":"{\"uuid\":\"fcb7ca4d-c34c-405a-9b30-93bcb1f19257\",\"targets\":\"172.30.24.144:9996\"}",
//!      "id":1
//! }
//! 

use super::ovs_common::*;

use jsonrpsee::ws_server::RpcModule;
use std::collections::HashMap;

use super::ovs_port::*;
use super::ovs_client::*;

use hmir_hash::HashWrap;
use hmir_token::TokenChecker;
use hmir_errno::errno;

use serde_json::json;

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
        
    module.register_method("ovs-query-connection", |params, _| {
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);
        Ok(check_connection())
    })?;
    
    module.register_method("ovs-query-ports", |params, _| {
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);
        Ok(get_ports())
    })?;

    module.register_method("ovs-query-bridges", |params, _| {
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);
        Ok(get_bridges())
    })?;

    module.register_method("ovs-query-interfaces", |params, _| {
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);
        Ok(get_interfaces())
    })?;

    module.register_method("ovs-query-netflows", |_, _| {
        Ok(get_netflow())
    })?;

    module.register_method("ovs-query-ipfix", |_, _| {
        Ok(get_ipfix())
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
            let ret_message = serde_json::to_string(&(e.error_detail.clone())).unwrap();
            ret_message
        },
        Ok(mut c) => {
            let is_connected = c.check_connection();
            if is_connected {
                "Connected!".to_string()
            } else {
                "Failed to connect".to_string()
            }
        }      
    }    
}

fn get_ports() -> std::string::String{
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            e.error_detail.clone()
        },
        Ok(mut c)=>{
            let ports = c.get_ports();
            match ports{
                Ok(ports) =>{
                    println!("number of port : {0}", ports.len());
                    let ret_message = serde_json::to_string(&ports).unwrap();
                    ret_message    
                },
                Err(e) => {
                    e.error_detail.clone()
                }
            }
        }
    }
}

fn get_bridges() -> std::string::String {
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            e.error_detail.clone()
        },
        Ok(mut c)=>{
            let bridges = c.get_bridges();
            match bridges{
                Ok(bridges) =>{
                    println!("number of bridge: {0}", bridges.len());
                    let ret_message = serde_json::to_string(&bridges).unwrap();
                    ret_message    
                },
                Err(e) => {
                    e.error_detail.clone()
                }
            }
        }
    }
}

fn get_interfaces() -> std::string::String{
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            e.error_detail.clone()
        },
        Ok(mut c)=>{
            let interfaces = c.get_interfaces();
            match interfaces{
                Ok(interfaces) =>{
                    println!("number of interfaces : {0}", interfaces.len());
                    let ret_message = serde_json::to_string(&interfaces).unwrap();
                    ret_message    
                },
                Err(e) => {
                    e.error_detail.clone()
                }
            }
        }
    }
}

fn get_netflow() -> std::string::String{
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            e.error_detail.clone()
        },
        Ok(mut c)=>{
            let netflow = c.get_netflows();
            match netflow{
                Ok(netflow) =>{
                
                    let ret_message = serde_json::to_string(&netflow).unwrap();
                    ret_message    
                },
                Err(e) => {
                    e.error_detail.clone()
                }
            }
        }
    }
}

fn get_ipfix() -> std::string::String{
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            e.error_detail.clone()
        },
        Ok(mut c)=>{
            let ipfix = c.get_ipfix();
            match ipfix{
                Ok(ipfix) =>{
                
                    let ret_message = serde_json::to_string(&ipfix).unwrap();
                    ret_message    
                },
                Err(e) => {
                    e.error_detail.clone()
                }
            }
        }
    }
}

fn add_port(info_map : HashMap<String, String>) -> std::string::String {
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            e.error_detail.clone()
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
                    e.error_detail.clone()
                }
            }
        }
    }
}

fn del_port(info_map : HashMap<String, String>) -> std::string::String {
    let ovsc = OvsClient::new();
    match ovsc{
        Err(e) => {
            e.error_detail.clone()
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
                    e.error_detail.clone()
                }
            }
        }
    }
}

// 由于测试网桥会在用例中不断被清理，需保证串行执行用例：cargo test  -- --test-threads=1 
#[cfg(test)]
mod query_tests{
    use super::*;

    #[test]
    fn test_query(){
        test_setup_env();

        assert_eq!(check_connection(), "Connected!".to_string());
        let mut ret_str = get_bridges();
        assert!(ret_str.contains(BR_FOR_TEST));

        ret_str = get_ports();
        assert!(ret_str.contains(BR_FOR_TEST));

        ret_str = get_interfaces();
        assert!(ret_str.contains(BR_FOR_TEST));

        let rule_netflow = format!("ovs-vsctl set Bridge {} netflow=@nf -- --id=@nf create NetFlow targets=\\\"172.30.21.13:2055\\\" active-timeout=60", BR_FOR_TEST);
        exec_rule(rule_netflow, "test_query rule_netflow".to_string());
        ret_str = get_netflow();
        assert!(ret_str.contains("172.30.21.13:2055"));

        let rule_ipfix = format!("ovs-vsctl set Bridge {} ipfix=@i -- --id=@i create IPFIX targets=\\\"172.30.21.14:2055\\\"", BR_FOR_TEST);
        exec_rule(rule_ipfix, "test_query rule_ipfix".to_string());
        ret_str = get_ipfix();
        assert!(ret_str.contains("172.30.21.14:2055"));

        test_clear_env();
    }

}
