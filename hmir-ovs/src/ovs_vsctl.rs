//! ovs-vsctl实现，网桥操作为主
//! 
//! 支持以下的格式
//! - ovs-add-br: 添加网桥
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-add-br" ,
//!     "params": {"br_name":"ovsmgmt"}
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
//! - ovs-del-br： 删除网桥
//! 请求格式：
//!  ```
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"ovs-del-br",
//!     "params": {"br_name":"ovsmgmt"} 
//! }
//!  ```
//! 
//! 响应格式：
//! ```
//! {
//!     "jsonrpc":"2.0",
//!     "result": "Done",
//!     "id":1
//! }
//! ```

use std::process::Command;
use std::collections::HashMap;

pub fn add_br(info_map : HashMap<String, String>) -> std::string::String {
    let br_name = info_map.get("br_name").unwrap();
    let output = Command::new("ovs-vsctl")
                                        .arg("add-br")
                                        .arg(br_name).
                                        output().expect("failed to excute ovs-add-br");
    if output.status.success() {
        String::from("Done")
    }else {
        String::from_utf8_lossy(&output.stderr).to_string()
    } 
}

pub fn del_br(info_map : HashMap<String, String>) -> std::string::String {
    let br_name = info_map.get("br_name").unwrap();
    let output = Command::new("ovs-vsctl")
                                        .arg("del-br")
                                        .arg(br_name).
                                        output().expect("failed to excute ovs-del-br");
    if output.status.success() {
        String::from("Done")
    }else {
        String::from_utf8_lossy(&output.stderr).to_string()
    }    
}
