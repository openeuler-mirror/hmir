//! ovs-vsctl实现
//! 
//! 支持以下的格式
//! - ovs-vsctl-add-br: 添加网桥
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-add-br" ,
//!     "params": {"br_name":"ovsmgmt"}
//! }
//! 
//! 响应格式：
//! {
//!     "jsonrpc":"2.0",
//!     "result":"Done",
//!     "id":1
//! }
//! 
//! - ovs-vsctl-del-br： 删除网桥
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-del-br",
//!     "params": {"br_name":"ovsmgmt"} 
//! }
//!
//! - ovs-vsctl-add-port： 网桥中添加端口
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-add-port",
//!     "params": {"br_name":"ovsmgmt", "port_name":"ens4"} 
//! }
//! 
//! - ovs-vsctl-del-port： 网桥中删除端口
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-del-port",
//!     "params": {"br_name":"ovsmgmt", "port_name": "ens4"} 
//! }
//! 
//! - ovs-vsctl-set-netflow-rule： 网桥中设置netflow 规则
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-set-netflow-rule",
//!     "params": {"br_name":"ovsmgmt", "targets": "172.30.24.3:2055"} 
//! }
//! 
//! - ovs-vsctl-del-netflow-rule： 网桥中删除netflow 规则
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-del-netflow-rule",
//!     "params": {"br_name":"ovsmgmt"} 
//! }
//! 
//! - ovs-vsctl-set-port-vlan： 设置ovs port vlanID
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"vs-vsctl-set-port-vlan",
//!     "params": {"port_name":"vnet0", "tag_value":"2"} 
//! }
//! 
//! - ovs-vsctl-set-ipfix-rule： 网桥中设置ipfix 规则
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-set-ipfix-rule",
//!     "params": {"br_name":"ovsmgmt", "targets": "172.30.24.3:2055"} 
//! }
//! 
//! - ovs-vsctl-del-ipfix-rule： 网桥中删除ipfix 规则
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-del-ipfix-rule",
//!     "params": {"br_name":"ovsmgmt"} 
//! } 

use super::ovs_common::*;
use std::collections::HashMap;
use jsonrpsee::ws_server::RpcModule;

const VSCTL_CMD: &str = "ovs-vsctl";

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("ovs-vsctl-add-br", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl_add_br(br_info))
    })?;

    module.register_method("ovs-vsctl-del-br", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl_del_br(br_info))
    })?;

    module.register_method("ovs-vsctl-add-port", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl_add_port(br_info))
    })?;

    module.register_method("ovs-vsctl-del-port", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl_del_port(br_info))
    })?;

    module.register_method("ovs-vsctl-set-netflow-rule", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl_set_netflow_rule(br_info))
    })?;

    module.register_method("ovs-vsctl-del-netflow-rule", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl_del_netflow_rule(br_info))
    })?;

    module.register_method("ovs-vsctl-set-ipfix-rule", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl_set_ipfix_rule(br_info))
    })?;

    module.register_method("ovs-vsctl-del-ipfix-rule", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl_del_ipfix_rule(br_info))
    })?;

    module.register_method("ovs-vsctl-set-port-vlan", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl_set_port_vlan(br_info))
    })?;

    Ok(())
}


fn ovs_vsctl_add_br(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} add-br {}", VSCTL_CMD, br_name);
    
    let output = exec_rule(rule, "ovs_vsctl_add_br".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_del_br(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} del-br {}", VSCTL_CMD, br_name);

    let output = exec_rule(rule, "ovs_vsctl_del_br".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_add_port(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let port_name = info_map.get("port_name").unwrap();
    let rule = format!("{} add-port {} {}", VSCTL_CMD, br_name, port_name);

    let output = exec_rule(rule, "ovs_vsctl_add_port".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_del_port(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let port_name = info_map.get("port_name").unwrap();
    let rule = format!("{} del-port {} {}", VSCTL_CMD, br_name, port_name);

    let output = exec_rule(rule, "ovs_vsctl_del_port".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_set_netflow_rule(info_map : HashMap<String, String>) -> std::string::String {
    let br_name = info_map.get("br_name").unwrap();
    let targets =  info_map.get("targets").unwrap();
    let rule = format!("{} set Bridge {} netflow=@nf -- --id=@nf create NetFlow targets=\\\"{}\\\" active-timeout=60", VSCTL_CMD, br_name, targets);
    
    let output = exec_rule(rule, "ovs_vsctl_set_netflow_rule".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_del_netflow_rule(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} clear Bridge {} netflow", VSCTL_CMD, br_name);
    
    let output = exec_rule(rule, "ovs_vsctl_del_netflow_rule".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_set_ipfix_rule(info_map : HashMap<String, String>) -> String{

    let br_name = info_map.get("br_name").unwrap();
    let targets =  info_map.get("targets").unwrap();
    let rule = format!("{} set Bridge {} ipfix=@i -- --id=@i create IPFIX targets=\\\"{}\\\"", 
                                VSCTL_CMD, br_name, targets);
    
    let output = exec_rule(rule, "ovs_vsctl_set_ipfix_rule".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_del_ipfix_rule(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} clear Bridge {} ipfix", VSCTL_CMD, br_name);
    
    let output = exec_rule(rule, "ovs_vsctl_del_ipfix_rule".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_set_port_vlan(info_map : HashMap<String, String>) -> String{
    let port_name = info_map.get("port_name").unwrap();
    let tag_value =  info_map.get("tag_value").unwrap();
    let rule = format!("{} set Port {} tag={}", VSCTL_CMD, port_name, tag_value);

    let output = exec_rule(rule, "ovs_vsctl_set_port_vlan".to_string());
    reflect_cmd_result(output)
}

// 由于测试网桥会在用例中不断被清理，需保证串行执行用例：cargo test  -- --test-threads=1 
#[cfg(test)]
mod vsctl_tests{
    use super::*;

    #[test]
    fn test_br(){
        test_clear_env();

        let mut br_info = HashMap::new();
        br_info.insert("br_name".to_string(), BR_FOR_TEST.to_string());
        
        assert_eq!(ovs_vsctl_add_br(br_info.clone()), "Done".to_string());
        assert_eq!(ovs_vsctl_del_br(br_info.clone()), "Done".to_string());

        test_clear_env();
    }

    #[test]
    fn test_port(){
        test_clear_env();
        
        let mut br_info = HashMap::new();
        br_info.insert("br_name".to_string(), BR_FOR_TEST.to_string());
        br_info.insert("port_name".to_string(), PORT_FOR_TEST.to_string());
        br_info.insert("tag_value".to_string(), "100".to_string());
        
        assert_eq!(ovs_vsctl_add_br(br_info.clone()), "Done".to_string());
        assert_eq!(ovs_vsctl_add_port(br_info.clone()), "Done".to_string());
        assert_eq!(ovs_vsctl_set_port_vlan(br_info.clone()), "Done".to_string());
        assert_eq!(ovs_vsctl_del_port(br_info.clone()), "Done".to_string());

        test_clear_env();
    }

    #[test]
    fn test_netflow(){
        test_setup_env();

        let mut br_info = HashMap::new();
        br_info.insert("br_name".to_string(), BR_FOR_TEST.to_string());
        br_info.insert("targets".to_string(), "172.30.24.122:2055".to_string());

        assert_eq!(ovs_vsctl_set_netflow_rule(br_info.clone()), "Done".to_string());
        assert_eq!(ovs_vsctl_del_netflow_rule(br_info.clone()), "Done".to_string());

        test_clear_env();
    }

    #[test]
    fn test_ipfix(){
        test_setup_env();

        let mut br_info = HashMap::new();
        br_info.insert("br_name".to_string(), BR_FOR_TEST.to_string());
        br_info.insert("targets".to_string(), "172.30.24.122:2055".to_string());

        assert_eq!(ovs_vsctl_set_ipfix_rule(br_info.clone()), "Done".to_string());
        assert_eq!(ovs_vsctl_del_ipfix_rule(br_info.clone()), "Done".to_string());

        test_clear_env();
    }
}