//! ovs-ofctl实现
//! 
//! 支持以下的格式
//! - ovs-ofctl-forbid-dstip: 禁止虚拟机访问外部某IP
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-forbid-dstip" ,
//!     "params": {"br_name":"ovsmgmt", "in_port":"vnet0", "dst_ip":"172.30.24.124"}
//! }
//!  ```
//! 响应格式：
//! ```
//! {
//!     "jsonrpc":"2.0",
//!     "result":"Done",
//!     "id":1
//! }
//! ```
//! - ovs-ofctl-clear-port-rules: 删除虚拟机网络接口的流表规则 
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-clear-port-rules" ,
//!     "params": {"br_name":"ovsmgmt", "in_port":"vnet0"}
//! }
//!  ```
//! - ovs-ofctl-add-default-rule:  为网桥设置默认流表规则，二层交换机能力
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-add-default-rule" ,
//!     "params": {"br_name":"ovsmgmt"}
//! }
//!  ```
//! - ovs-ofctl-forbid-dstport:  禁止虚拟机访问外部端口
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-forbid-dstport" ,
//!     "params": {"br_name":"ovsmgmt", "in_port":"vnet0", "dst_ip":"172.30.24.124","dst_port":"8080/0xffff"}
//! }
//!  ```
//! - ovs-ofctl-pass-dstip:  放通访问外部Ip地址，白名单能力
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-pass-dstip" ,
//!     "params": {"br_name":"ovsmgmt", "in_port":"vnet0", "dst_ip":"172.30.24.0/24",}
//! }
//!  ```
//! - ovs-ofctl-pass-dstport:  放通访问外部Ip地址的某端口，白名单能力
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-pass-dstport" ,
//!     "params": {"br_name":"ovsmgmt", "in_port":"vnet0", "dst_ip":"172.30.24.124","dst_port":"8080/0xffff"}
//! }
//!  ```

use super::ovs_common::*;

use std::process::{Command};
use std::collections::HashMap;
use jsonrpsee::ws_server::RpcModule;

const OFCTL_CMD: &str= "ovs-ofctl";
const OFCTL_PRIO_BLK: &str= "1";
const OFCTL_PRIO_WHI: &str= "2";
const OFCTL_PRIO_GROUP: &str= "10";
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

    Ok(())
}

fn ovs_ofctl_clear_port_rules(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let in_port = info_map.get("in_port").unwrap();
    let clear_rule = format!("in_port={}", in_port);
    let output = Command::new(OFCTL_CMD)
                                        .arg("del-flows")
                                        .arg(br_name)
                                        .arg(clear_rule)
                                        .output().expect("failed to execute ovs_ofctl_clear_port_rules");
    reflect_cmd_result(output)
}

fn ovs_ofctl_forbid_dstip(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let dst_ip = info_map.get("dst_ip").unwrap();
    let in_port = info_map.get("in_port").unwrap();
    
    let mut flow_rule = String::new();
    if in_port.is_empty() {
        flow_rule = format!("dl_type=0x0800,nw_dst={},priority={},action=drop", dst_ip, OFCTL_PRIO_BLK);
    } else {
        flow_rule = format!("dl_type=0x0800,nw_dst={},priority={},in_port={},action=drop", dst_ip, OFCTL_PRIO_BLK,in_port);
    }

    let output = Command::new(OFCTL_CMD)
                                        .arg("add-flow")
                                        .arg(br_name)
                                        .arg(flow_rule).
                                        output().expect("failed to excute ovs-ofctl-forbid-dstip");
    reflect_cmd_result(output)
}

fn ovs_ofctl_add_default_rule(info_map : HashMap<String, String>) -> String {
    let br_name = info_map.get("br_name").unwrap();
    let output = Command::new(OFCTL_CMD)
                                        .arg("add-flow")
                                        .arg(br_name)
                                        .arg("priority=0,action=normal")
                                        .output().expect("failed to execute ovs_ofctl_add_default_rule");
    
    reflect_cmd_result(output)
}


fn ovs_ofctl_forbid_dstport(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let dst_ip = info_map.get("dst_ip").unwrap();
    let in_port = info_map.get("in_port").unwrap();
    let dst_port = info_map.get("dst_port").unwrap();
    let flow_tcp = format!("dl_type=0x0800,nw_proto=6,nw_dst={},tp_dst={},in_port={},priority={},action=drop", 
                                    dst_ip, dst_port, in_port, OFCTL_PRIO_BLK);
    
    let mut output = Command::new(OFCTL_CMD)
                                        .arg("add-flow")
                                        .arg(br_name)
                                        .arg(flow_tcp)
                                        .output().expect("failed to execute ovs_ofctl_add_default_rule_tcp");
    if !output.status.success() {
        return String::from_utf8_lossy(&output.stderr).to_string()
    }

    let flow_udp = format!("dl_type=0x0800,nw_proto=17,nw_dst={},tp_dst={},in_port={},priority={},action=drop", 
                                        dst_ip, dst_port, in_port, OFCTL_PRIO_BLK);

    output = Command::new(OFCTL_CMD)
                                        .arg("add-flow")
                                        .arg(br_name)
                                        .arg(flow_udp)
                                        .output().expect("failed to execute ovs_ofctl_add_default_rule_udp");
    reflect_cmd_result(output)

}


// 先禁用端口所有的tcp请求，再放通白名单中的tcp请求
fn ovs_ofctl_pass_dstip(info_map : HashMap<String, String>) ->String{
    let br_name = info_map.get("br_name").unwrap();
    let dst_ip = info_map.get("dst_ip").unwrap();
    let in_port = info_map.get("in_port").unwrap();

    let flow_tcp_forbid= format!("dl_type=0x0800,in_port={},priority={},action=drop", in_port, OFCTL_PRIO_BLK);
    let mut output = Command::new(OFCTL_CMD)
                                    .arg("add-flow")
                                    .arg(br_name)
                                    .arg(flow_tcp_forbid)
                                    .output().expect("failed to execute flow_tcp_forbid");
    if !output.status.success() {                                    
        println!("execute flow_tcp_forbid exception : {}",  String::from_utf8_lossy(&output.stderr));
    }

    let flow_tcp_pass = format!("dl_type=0x0800,nw_dst={},in_port={},priority={},action=drop", dst_ip, in_port, OFCTL_PRIO_WHI);
    output = Command::new(OFCTL_CMD)
                        .arg("add-flow")
                        .arg(br_name)
                        .arg(flow_tcp_pass)
                        .output().expect("failed to execute flow_tcp_pass");
    reflect_cmd_result(output)

}

fn ovs_ofctl_pass_dstport(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let dst_ip = info_map.get("dst_ip").unwrap();
    let in_port = info_map.get("in_port").unwrap();
    let dst_port = info_map.get("dst_port").unwrap();
    
    let flow_forbid_dstip = format!("dl_type=0x0800,nw_dst={},priority={},in_port={},action=drop", dst_ip, OFCTL_PRIO_BLK,in_port);
    let mut output = Command::new(OFCTL_CMD)
                                        .arg("add-flow")
                                        .arg(br_name)
                                        .arg(flow_forbid_dstip).
                                         output().expect("failed to excute ovs-ofctl-flow-forbid-dstip");
    if !output.status.success() {
            return String::from_utf8_lossy(&output.stderr).to_string()
    }
    
    let flow_tcp_white = format!("dl_type=0x0800,nw_proto=6,nw_dst={},tp_dst={},in_port={},priority={},action=normal", 
                                    dst_ip, dst_port, in_port, OFCTL_PRIO_WHI);
    
    output = Command::new(OFCTL_CMD)
                                        .arg("add-flow")
                                        .arg(br_name)
                                        .arg(flow_tcp_white)
                                        .output().expect("failed to execute ovs_ofctl_add_default_rule_tcp_whie");
    if !output.status.success() {
        return String::from_utf8_lossy(&output.stderr).to_string()
    }

    let flow_udp_white = format!("dl_type=0x0800,nw_proto=17,nw_dst={},tp_dst={},in_port={},priority={},action=normal", 
                                        dst_ip, dst_port, in_port, OFCTL_PRIO_WHI);

    output = Command::new(OFCTL_CMD)
                                        .arg("add-flow")
                                        .arg(br_name)
                                        .arg(flow_udp_white)
                                        .output().expect("failed to execute ovs_ofctl_add_default_rule_udp_white");
    reflect_cmd_result(output)

}
