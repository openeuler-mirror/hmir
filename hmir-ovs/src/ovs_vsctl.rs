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
//! 
//! - ovs-vsctl-set-interface-policing：设置接口限流policing， rate单位kbps, burst单位kb
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-set-interface-policing",
//!     "params": {"interface_name":"vnet0", "rate":"1000", "burst":"100"} 
//! }
//! 
//! - ovs-vsctl-set-port-qos：设置port qos策略, max-rate单位bps
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-set-port-qos",
//!     "params": {"interface_name":"vnet0", "qos_type":"linux-htb",ovs_vsctl_add_port "max-rate":"1000000"} 
//! }
//! 
//! - ovs-vsctl-set-port-patch：设置ovs网桥间的连接patch口
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-set-port-patch",
//!     "params": {"br_name":"ovsmgmt", "port_name":"patch1", "peer_name":"patch2"} 
//! }
//! 
//! - ovs-vsctl-set-port-bond: ovs网桥添加bond接口
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-vsctl-set-port-bond",
//!     "params": {"br_name":"ovsmgmt", "bond_name":"bond1", "nics":"eth1,eth2,eth3"} 
//! }

use super::ovs_common::*;
use jsonrpsee::{ws_server::RpcModule};

use serde_json::{json, Value};
use std::collections::BTreeMap;

use hmir_hash::HashWrap;
use hmir_token::TokenChecker;
use hmir_errno::errno;

use crate::OvsTokenChecker;

const VSCTL_CMD: &str = "ovs-vsctl";

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("ovs-vsctl-add-br", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        OvsTokenChecker!(br_info);
        Ok(ovs_vsctl_add_br(br_info))
    })?;

    module.register_method("ovs-vsctl-del-br", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        OvsTokenChecker!(br_info);
        Ok(ovs_vsctl_del_br(br_info))
    })?;

    module.register_method("ovs-vsctl-add-port", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        OvsTokenChecker!(br_info);
        Ok(ovs_vsctl_add_port(br_info))
    })?;

    module.register_method("ovs-vsctl-del-port", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        OvsTokenChecker!(br_info);
        Ok(ovs_vsctl_del_port(br_info))
    })?;

    module.register_method("ovs-vsctl-set-netflow-rule", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        Ok(ovs_vsctl_set_netflow_rule(br_info))
    })?;

    module.register_method("ovs-vsctl-del-netflow-rule", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        Ok(ovs_vsctl_del_netflow_rule(br_info))
    })?;

    module.register_method("ovs-vsctl-set-ipfix-rule", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        Ok(ovs_vsctl_set_ipfix_rule(br_info))
    })?;

    module.register_method("ovs-vsctl-del-ipfix-rule", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        Ok(ovs_vsctl_del_ipfix_rule(br_info))
    })?;

    module.register_method("ovs-vsctl-set-port-vlan", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        Ok(ovs_vsctl_set_port_vlan(br_info))
    })?;

    module.register_method("ovs-vsctl-set-interface-policing", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        Ok(ovs_vsctl_set_interface_policing(br_info))
    })?;

    module.register_method("ovs-vsctl-set-port-qos", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        Ok(ovs_vsctl_set_port_qos(br_info))
    })?;

    module.register_method("ovs-vsctl-set-port-patch", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        Ok(ovs_vsctl_set_port_patch(br_info))
    })?;

    module.register_method("ovs-vsctl-set-port-bond", |params, _| {
        let br_info = params.parse::<BTreeMap<&str, Value>>()?;
        Ok(ovs_vsctl_set_port_bond(br_info))
    })?;

    Ok(())
}


fn ovs_vsctl_add_br(info_map : BTreeMap<&str, Value>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    println!("br_name:{}", br_name);
    let rule = format!("{} add-br {}", VSCTL_CMD, br_name);

    let output = exec_rule(rule, "ovs_vsctl_add_br".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_del_br(info_map : BTreeMap<&str, Value>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} del-br {}", VSCTL_CMD, br_name);

    let output = exec_rule(rule, "ovs_vsctl_del_br".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_add_port(info_map : BTreeMap<&str, Value>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let port_name = info_map.get("port_name").unwrap();
    let rule = format!("{} add-port {} {}", VSCTL_CMD, br_name, port_name);

    let output = exec_rule(rule, "ovs_vsctl_add_port".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_del_port(info_map : BTreeMap<&str, Value>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let port_name = info_map.get("port_name").unwrap();
    let rule = format!("{} del-port {} {}", VSCTL_CMD, br_name, port_name);

    let output = exec_rule(rule, "ovs_vsctl_del_port".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_set_netflow_rule(info_map : BTreeMap<&str, Value>) -> std::string::String {
    let br_name = info_map.get("br_name").unwrap();
    let targets =  info_map.get("targets").unwrap();
    let rule = format!("{} set Bridge {} netflow=@nf -- --id=@nf create NetFlow targets=\\\"{}\\\" active-timeout=60", VSCTL_CMD, br_name, targets);
    
    let output = exec_rule(rule, "ovs_vsctl_set_netflow_rule".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_del_netflow_rule(info_map : BTreeMap<&str, Value>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} clear Bridge {} netflow", VSCTL_CMD, br_name);
    
    let output = exec_rule(rule, "ovs_vsctl_del_netflow_rule".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_set_ipfix_rule(info_map : BTreeMap<&str, Value>) -> String{

    let br_name = info_map.get("br_name").unwrap();
    let targets =  info_map.get("targets").unwrap();
    let rule = format!("{} set Bridge {} ipfix=@i -- --id=@i create IPFIX targets=\\\"{}\\\"", 
                                VSCTL_CMD, br_name, targets);
    
    let output = exec_rule(rule, "ovs_vsctl_set_ipfix_rule".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_del_ipfix_rule(info_map : BTreeMap<&str, Value>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} clear Bridge {} ipfix", VSCTL_CMD, br_name);
    
    let output = exec_rule(rule, "ovs_vsctl_del_ipfix_rule".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_set_port_vlan(info_map : BTreeMap<&str, Value>) -> String{
    let port_name = info_map.get("port_name").unwrap();
    let tag_value =  info_map.get("tag_value").unwrap();
    let rule = format!("{} set Port {} tag={}", VSCTL_CMD, port_name, tag_value);

    let output = exec_rule(rule, "ovs_vsctl_set_port_vlan".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_set_interface_policing(info_map : BTreeMap<&str, Value>) -> String{
    let interface_name = info_map.get("interface_name").unwrap();
    let rate =  info_map.get("rate").unwrap();
    let burst = info_map.get("burst").unwrap();
    let rule_rate = format!("{} set interface {} ingress_policing_rate={}", VSCTL_CMD, interface_name, rate);
    let mut output = exec_rule(rule_rate, "ovs_vsctl_set_interface_policing rule_rate".to_string());
    if !output.status.success() {
        println!("execute ovs_vsctl_set_interface_policing rule_rate exception : {}",  String::from_utf8_lossy(&output.stderr));
    }

    let rule_burst = format!("{} set interface {} ingress_policing_burst={}", VSCTL_CMD, interface_name, burst);
    output = exec_rule(rule_burst, "ovs_vsctl_set_interface_policing rule_burst".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_set_port_qos(info_map : BTreeMap<&str, Value>) -> String{
    let port_name = info_map.get("port_name").unwrap();
    let qos_type = info_map.get("qos_type").unwrap();
    let max_rate = info_map.get("max_rate").unwrap();

    let rule = format!("{} set Port {} qos=@newqos -- \
                                --id=@newqos create qos type={} queues=0=@q0 -- \
                                --id=@q0 create queue other-config:max-rate={}", 
                                VSCTL_CMD, port_name, qos_type, max_rate);
    let output = exec_rule(rule, "ovs_vsctl_set_port_qos".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_set_port_patch(info_map : BTreeMap<&str, Value>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let port_name = info_map.get("port_name").unwrap();
    let peer_port = info_map.get("peer_port").unwrap();
    let rule = format!("{} add-port {} {port} -- \
                                    set Interface {port} type=patch -- \
                                    set Interface {port} options:peer={peer}", 
                                    VSCTL_CMD, br_name, port=port_name, peer=peer_port);
    
    let output = exec_rule(rule, "ovs_vsctl_set_port_patch".to_string());
    reflect_cmd_result(output)
}

fn ovs_vsctl_set_port_bond(info_map : BTreeMap<&str, Value>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let bond_name = info_map.get("bond_name").unwrap();
    let nics = info_map.get("nics").unwrap().to_string();
    let nic_vec : Vec<&str> = nics.split(",").collect();

    let mut nic_str = String::new();
    for s in nic_vec.iter(){
        nic_str = nic_str + s + " ";
    }

    let rule =  format!("{} add-bond {} {} {}", VSCTL_CMD, br_name, bond_name, nic_str);

    let output = exec_rule(rule, "ovs_vsctl_set_port_patch".to_string());
    reflect_cmd_result(output)
}

// 由于测试网桥会在用例中不断被清理，需保证串行执行用例：cargo test  -- --test-threads=1 
#[cfg(test)]
mod vsctl_tests{
    use super::*;

    #[test]
    fn test_br(){
        test_clear_env();

        let mut br_info = BTreeMap::new();
        br_info.insert("br_name", json!(BR_FOR_TEST));
        
        assert_eq!(ovs_vsctl_add_br(br_info.clone()), "Done".to_string());
        assert_eq!(ovs_vsctl_del_br(br_info.clone()), "Done".to_string());

        test_clear_env();
    }

    #[test]
    fn test_port(){
        test_clear_env();
        
        let mut br_info = BTreeMap::new();
        br_info.insert("br_name", json!(BR_FOR_TEST));
        
        assert_eq!(ovs_vsctl_add_br(br_info.clone()), "Done".to_string());
        br_info.insert("port_name", json!(PORT_FOR_TEST));
        assert_eq!(ovs_vsctl_add_port(br_info.clone()), "Done".to_string());
        
        br_info.insert("tag_value", json!("100"));
        assert_eq!(ovs_vsctl_set_port_vlan(br_info.clone()), "Done".to_string()); 

        br_info.insert("qos_type", json!("linux-htb"));
        br_info.insert("max_rate", json!("1000000"));
        assert_eq!(ovs_vsctl_set_port_qos(br_info.clone()), "Done".to_string());

        br_info.insert("port_name", json!("patch1"));
        br_info.insert("peer_port", json!("patch2"));
        assert_eq!(ovs_vsctl_set_port_patch(br_info.clone()), "Done".to_string());

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

    #[test]
    fn test_interface(){
        test_setup_env();

        let mut br_info = HashMap::new();
        br_info.insert("interface_name".to_string(), BR_FOR_TEST.to_string());
        br_info.insert("rate".to_string(), "1000".to_string());
        br_info.insert("burst".to_string(), "100".to_string());

        assert_eq!(ovs_vsctl_set_interface_policing(br_info.clone()), "Done".to_string());
        test_clear_env();
    }
}