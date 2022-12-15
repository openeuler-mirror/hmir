//! ovs-ofctl实现
//! 
//! 支持以下的格式
//! - ovs-ofctl-forbid-dstip: 
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-ofctl-forbid-dstip" ,
//!     "params": {"br_name":"ovsmgmt", "dst_ip":"1722.30.24.124", "in_port":"vnet0"}
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
//! 

use std::process::Command;
use std::collections::HashMap;
use jsonrpsee::ws_server::RpcModule;

const OFCTL_CMD: &str= "ovs-ofctl";
const OFCTL_PRIO_BLK: &str= "1";
const OFCTL_PRIO_WHI: &str= "2";
const OFCTL_PRIO_GROUP: &str= "10";
const OFCTL_PRIO_SINGLE: &str= "20";

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{

    module.register_method("ovs-ofctl-forbid-dstip", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(forbid_dstip(br_info))
    })?;

    Ok(())
}

fn forbid_dstip(info_map : HashMap<String, String>) -> std::string::String {
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
                                        output().expect("failed to excute ovs-vsctl-add-port");
    if output.status.success() {
        String::from("Done")
    }else {
        String::from_utf8_lossy(&output.stderr).to_string()
    } 
}