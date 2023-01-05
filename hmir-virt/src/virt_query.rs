//! - virt-check-connection: virt 检查虚拟机连接
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-check-connection"
//! }
//! 
//! //! - virt-show-hypervisor: virt 展示hypervisor信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-hypervisor"
//! }

const QEMU_URI: &str= "qemu:///system";
use  virt::connect::Connect;
use std::collections::HashMap;
use super::virt_type::*;

use jsonrpsee::ws_server::RpcModule;


pub fn register_virt_query(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("virt-check-connection", |params, _| {
        let info = params.parse::<HashMap<String, String>>()?;
        Ok(virt_check_connection(info))
    })?;

    module.register_method("virt-show-hypervisor", |params, _| {
        let info = params.parse::<HashMap<String, String>>()?;
        Ok(virt_show_hypervisor(info))
    })?;

    Ok(())
}

fn virt_check_connection(info: HashMap<String, String>) -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(mut c) => {
            c
        },
        Err(e) => panic!("Not connected, code: {}, message: {}", e.code, e.message), 
    };
    
    match conn.close() {
        Ok(_) => { "Connection Finished Normally".to_string() },
        Err(e) => panic!("Failed to disconnect from hypervisor: code {}, message: {}",
        e.code,
        e.message),
    }
}

fn virt_show_hypervisor(info: HashMap<String, String>) -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(mut c) => {
            c
        },
        Err(e) => panic!("Not connected, code: {}, message: {}", e.code, e.message), 
    };

    let mut ret_info = String::new();

    if let Ok(hv_type) = conn.get_type() {
        if let Ok(mut hv_ver) = conn.get_hyp_version() {
            let major = hv_ver / 1000000;
            hv_ver %= 1000000;
            let minor = hv_ver / 1000;
            let release = hv_ver % 1000;
            let hv_ver_str = format!("{}.{}.{}", major, minor, release);
            ret_info = serde_json::to_string(&HmirHvisor::new(hv_type, hv_ver_str)).unwrap();
        }
    }

    match conn.close() {
        Ok(_) => { ret_info },
        Err(e) => panic!("Failed to disconnect from hypervisor: code {}, message: {}",
        e.code,
        e.message),
    }
}

fn virt_show_domains(info: HashMap<String, String>) -> String{
    String::new()
}