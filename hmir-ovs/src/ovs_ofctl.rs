//! ovs-ofctl实现
//! 
//! 支持以下的格式
//! - ovs-ofctl-forbid-dstip: 禁止虚拟机访问外部某IP
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-forbid-dstip" ,
//!     "params": {"br_name":"ovsmgmt", "in_port":"vnet0", "dst_ip":"172.30.24.124"}
//! }
//! 响应格式：
//! {
//!     "jsonrpc":"2.0",
//!     "result":"Done",
//!     "id":1
//! }
//! 
//! - ovs-ofctl-clear-port-rules: 删除虚拟机网络接口的流表规则 
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-clear-port-rules" ,
//!     "params": {"br_name":"ovsmgmt", "in_port":"vnet0"}
//! }
//! 
//! - ovs-ofctl-add-default-rule:  为网桥设置默认流表规则，二层交换机能力
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-add-default-rule" ,
//!     "params": {"br_name":"ovsmgmt"}
//! }
//! 
//! - ovs-ofctl-forbid-dstport:  禁止虚拟机访问外部端口
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-forbid-dstport" ,
//!     "params": {"br_name":"ovsmgmt", "in_port":"vnet0", "dst_ip":"172.30.24.124","dst_port":"8080/0xffff"}
//! }
//! 
//! - ovs-ofctl-pass-dstip:  放通访问外部Ip地址，白名单能力
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-pass-dstip" ,
//!     "params": {"br_name":"ovsmgmt", "in_port":"vnet0", "dst_ip":"172.30.24.0/24",}
//! }
//! 
//! - ovs-ofctl-pass-dstport:  放通访问外部Ip地址的某端口，白名单能力
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-pass-dstport" ,
//!     "params": {"br_name":"ovsmgmt", "in_port":"vnet0", "dst_ip":"172.30.24.124","dst_port":"8080/0xffff"}
//! }
//! 
//! - ovs-ofctl-mod-vlan:  设置vlan id转换
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-mod-vlan" ,
//!     "params": {"br_name":"ovsmgmt", "in_port":"vnet0", "dl_valn":"100","mod_vlan":"10"}
//! }
//! 

use super::ovs_common::*;

use std::collections::HashMap;
use jsonrpsee::ws_server::RpcModule;

const OFCTL_CMD: &str= "ovs-ofctl";
const OFCTL_PRIO_BLK: &str= "1";
const OFCTL_PRIO_WHI: &str= "2";

#[allow(dead_code)]
const OFCTL_PRIO_GROUP: &str= "10";

#[allow(dead_code)]
const OFCTL_PRIO_SINGLE: &str= "20";

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{

    module.register_method("ovs-ofctl-add-default-rule", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_ofctl_add_default_rule(br_info))
    })?;
    
    module.register_method("ovs-ofctl-clear-port-rules", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_ofctl_clear_port_rules(br_info))
    })?;

    module.register_method("ovs-ofctl-forbid-dstip", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_ofctl_forbid_dstip(br_info))
    })?;

    module.register_method("ovs-ofctl-forbid-dstport", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_ofctl_forbid_dstport(br_info))
    })?;

    module.register_method("ovs-ofctl-pass-dstip", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_ofctl_pass_dstip(br_info))
    })?;

    module.register_method("ovs-ofctl-pass-dstport", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_ofctl_pass_dstport(br_info))
    })?;

    module.register_method("ovs-ofctl-mod-vlan", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_ofctl_mod_vlan(br_info))
    })?;

    Ok(())
}

#[allow(dead_code)]
fn ovs_ofctl_clear_br_rules(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} del-flows {}", OFCTL_CMD, br_name);
    
    let output = exec_rule(rule, "ovs_ofctl_clear_br_rules".to_string());
    reflect_cmd_result(output)
}

fn ovs_ofctl_clear_port_rules(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let in_port = info_map.get("in_port").unwrap();
    let rule = format!("{} del-flows {} in_port={}", OFCTL_CMD, br_name, in_port);
    
    let output = exec_rule(rule, "ovs_ofctl_clear_port_rules".to_string());
    reflect_cmd_result(output)
}

fn ovs_ofctl_forbid_dstip(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let dst_ip = info_map.get("dst_ip").unwrap();
    let in_port = info_map.get("in_port").unwrap();

    let rule = match in_port.is_empty() {
        true => {
            format!("{} add-flow {} dl_type=0x0800,nw_dst={},priority={},action=drop",OFCTL_CMD, br_name, dst_ip, OFCTL_PRIO_BLK)
        },
        false => {
            format!("{} add-flow {} dl_type=0x0800,nw_dst={},priority={},in_port={},action=drop", OFCTL_CMD, br_name, dst_ip, OFCTL_PRIO_BLK,in_port)
        }
    };

    let output = exec_rule(rule, "ovs_ofctl_forbid_dstip".to_string());
    reflect_cmd_result(output)
}

fn ovs_ofctl_add_default_rule(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} add-flow {} priority=0,action=normal", OFCTL_CMD, br_name);
    
    let output = exec_rule(rule, "ovs_ofctl_add_default_rule".to_string());
    reflect_cmd_result(output)
}

fn ovs_ofctl_forbid_dstport(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let dst_ip = info_map.get("dst_ip").unwrap();
    let in_port = info_map.get("in_port").unwrap();
    let dst_port = info_map.get("dst_port").unwrap();
    
    let rule_tcp = format!("{} add-flow {} dl_type=0x0800,nw_proto=6,nw_dst={},tp_dst={},in_port={},priority={},action=drop", 
                                    OFCTL_CMD, br_name, dst_ip, dst_port, in_port, OFCTL_PRIO_BLK);
    let mut output = exec_rule(rule_tcp, "ovs_ofctl_forbid_dstport add rule_tcp".to_string());
    if !output.status.success() {
        return String::from_utf8_lossy(&output.stderr).to_string()
    }

    let rule_udp = format!("{} add-flow {} dl_type=0x0800,nw_proto=17,nw_dst={},tp_dst={},in_port={},priority={},action=drop", 
                                    OFCTL_CMD,  br_name, dst_ip, dst_port, in_port, OFCTL_PRIO_BLK);
    output =  exec_rule(rule_udp, "ovs_ofctl_forbid_dstport add rule_udp".to_string());
    reflect_cmd_result(output)
}

// 先禁用端口所有的tcp请求，再放通白名单中的tcp请求
fn ovs_ofctl_pass_dstip(info_map : HashMap<String, String>) ->String{
    let br_name = info_map.get("br_name").unwrap();
    let dst_ip = info_map.get("dst_ip").unwrap();
    let in_port = info_map.get("in_port").unwrap();

    let rule_tcp_forbid= format!("{} add-flow {} dl_type=0x0800,in_port={},priority={},action=drop", 

                                            OFCTL_CMD, br_name, in_port, OFCTL_PRIO_BLK);
    let mut output = exec_rule(rule_tcp_forbid, "ovs_ofctl_pass_dstip add rule_tcp_forbid".to_string());
    if !output.status.success() {                                    
        println!("execute ovs_ofctl_pass_dstip exception : {}",  String::from_utf8_lossy(&output.stderr));
    }

    let rule_tcp_pass = format!("{} add-flow {} dl_type=0x0800,nw_dst={},in_port={},priority={},action=drop", 
                                            OFCTL_CMD, br_name, dst_ip, in_port, OFCTL_PRIO_WHI);
    output = exec_rule(rule_tcp_pass, "ovs_ofctl_pass_dstip add rule_tcp_pass".to_string());
    reflect_cmd_result(output)

}

fn ovs_ofctl_pass_dstport(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let dst_ip = info_map.get("dst_ip").unwrap();
    let in_port = info_map.get("in_port").unwrap();
    let dst_port = info_map.get("dst_port").unwrap();
    
    let rule_forbid_dstip = format!("{} add-flow {} dl_type=0x0800,nw_dst={},priority={},in_port={},action=drop",
                                                OFCTL_CMD, br_name, dst_ip, OFCTL_PRIO_BLK,in_port);
    let mut output = exec_rule(rule_forbid_dstip, "ovs_ofctl_pass_dstport add rule_forbid_dstip".to_string());
    if !output.status.success() {
            return String::from_utf8_lossy(&output.stderr).to_string()
    }
    
    let rule_tcp_white = format!("{} add-flow {} dl_type=0x0800,nw_proto=6,nw_dst={},tp_dst={},in_port={},priority={},action=normal", 
                                            OFCTL_CMD, br_name, dst_ip, dst_port, in_port, OFCTL_PRIO_WHI);
    output = exec_rule(rule_tcp_white, "ovs_ofctl_pass_dstport add rule_tcp_white".to_string());
    if !output.status.success() {
        return String::from_utf8_lossy(&output.stderr).to_string()
    }

    let rule_udp_white = format!("{} add-flow {} dl_type=0x0800,nw_proto=17,nw_dst={},tp_dst={},in_port={},priority={},action=normal", 
                                            OFCTL_CMD, br_name, dst_ip, dst_port, in_port, OFCTL_PRIO_WHI);

    output = exec_rule(rule_udp_white, "ovs_ofctl_pass_dstport add rule_udp_white".to_string());
    reflect_cmd_result(output)

}

fn ovs_ofctl_mod_vlan(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let in_port = info_map.get("in_port").unwrap();
    let dl_vlan = info_map.get("dl_vlan").unwrap();
    let mod_vlan = info_map.get("mod_vlan").unwrap();

    let rule = format!("{} add-flow {} in_port={},dl_vlan={},actions=mod_vlan_vid:{},normal",
                                OFCTL_CMD, br_name, in_port, dl_vlan, mod_vlan);
    let output = exec_rule(rule, "ovs_ofctl_mod_vlan".to_string());
    reflect_cmd_result(output)
}

// 由于测试网桥会在用例中不断被清理，需保证串行执行用例：cargo test  -- --test-threads=1 
#[cfg(test)]
mod ofctl_tests{
    use super::*;

    #[test]
    fn test_add_default_rule(){
        test_setup_env();

        let mut br_info = HashMap::new();
        br_info.insert("br_name".to_string(), BR_FOR_TEST.to_string());

        assert_eq!(ovs_ofctl_clear_br_rules(br_info.clone()), "Done".to_string());
        assert_eq!(ovs_ofctl_add_default_rule(br_info.clone()), "Done".to_string());
        
        test_clear_env();
    }

    #[test]
    fn test_forbid_rule(){
        test_setup_env();

        let mut br_info = HashMap::new();
        br_info.insert("br_name".to_string(), BR_FOR_TEST.to_string());
        br_info.insert("in_port".to_string(), BR_FOR_TEST.to_string());
        br_info.insert("dst_ip".to_string(), "172.30.24.124".to_string());
        
        assert_eq!(ovs_ofctl_forbid_dstip(br_info.clone()), "Done".to_string());
        assert_eq!(ovs_ofctl_clear_port_rules(br_info.clone()), "Done".to_string());

        br_info.insert("dst_port".to_string(), "8080/0xffff".to_string());
        assert_eq!(ovs_ofctl_forbid_dstport(br_info.clone()), "Done".to_string());

        test_clear_env();
    }

    #[test]
    fn test_pass_rule(){
        test_setup_env();

        let mut br_info = HashMap::new();
        br_info.insert("br_name".to_string(), BR_FOR_TEST.to_string());
        br_info.insert("in_port".to_string(), BR_FOR_TEST.to_string());
        br_info.insert("dst_ip".to_string(), "172.30.24.124".to_string());

        assert_eq!(ovs_ofctl_pass_dstip(br_info.clone()), "Done".to_string());
        assert_eq!(ovs_ofctl_clear_port_rules(br_info.clone()), "Done".to_string());

        br_info.insert("dst_port".to_string(), "8080/0xffff".to_string());
        assert_eq!(ovs_ofctl_pass_dstport(br_info.clone()), "Done".to_string());

        test_clear_env();
    }

    #[test]
    fn test_vlan_rule(){
        test_setup_env();

        let mut br_info = HashMap::new();
        br_info.insert("br_name".to_string(), BR_FOR_TEST.to_string());
        br_info.insert("in_port".to_string(), BR_FOR_TEST.to_string());
        br_info.insert("dl_vlan".to_string(), "100".to_string());
        br_info.insert("mod_vlan".to_string(), "10".to_string());

        assert_eq!(ovs_ofctl_mod_vlan(br_info.clone()), "Done".to_string());

        test_clear_env();
    }
}