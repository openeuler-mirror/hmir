//! - virt-check-connection: virt 检查虚拟机连接
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-check-connection"
//! }
//! 
//! - virt-show-hypervisor: virt 展示hypervisor信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-hypervisor"
//! }
//! 
//! - virt-show-domains: virt 展示active domains信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-domains"
//! }
//! 
//! - virt-show-uri: virt 展示uri信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-uri"
//! }
//!

use  virt::connect::Connect;
use std::collections::HashMap;
use super::virt_type::*;

use jsonrpsee::ws_server::RpcModule;

const QEMU_URI: &str= "qemu:///system";

pub fn register_virt_query(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("virt-check-connection", |params, _| {
        let info = params.parse::<HashMap<String, String>>()?;
        Ok(virt_check_connection(info))
    })?;

    module.register_method("virt-show-hypervisor", |params, _| {
        let info = params.parse::<HashMap<String, String>>()?;
        Ok(virt_show_hypervisor(info))
    })?;

    module.register_method("virt-show-domains", |_, _| {
        Ok(virt_show_domains())
    })?;

    module.register_method("virt-show-uri", |_, _| {
        Ok(virt_show_uri())
    })?;

    Ok(())
}

fn virt_check_connection(info: HashMap<String, String>) -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(mut c) => {
            c
        },
        Err(e) => return format!("Not connected, code: {}, message: {}", e.code, e.message), 
    };
    
    match conn.close() {
        Ok(_) => { "Connection Finished Normally".to_string() },
        Err(e) => format!("Failed to disconnect from hypervisor: code {}, message: {}",
        e.code,
        e.message),
    }
}

fn virt_show_hypervisor(info: HashMap<String, String>) -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(mut c) => {
            c
        },
        Err(e) => return format!("Not connected, code: {}, message: {}", e.code, e.message), 
    };

    let mut hv_info = HmirHvisor::default();

    if let Ok(hv_type) = conn.get_type() {
        if let Ok(mut hv_ver) = conn.get_hyp_version() {
            let major = hv_ver / 1000000;
            hv_ver %= 1000000;
            let minor = hv_ver / 1000;
            let release = hv_ver % 1000;
            let hv_ver_str = format!("{}.{}.{}", major, minor, release);
            hv_info = HmirHvisor::new(hv_type, hv_ver_str);
        }
    }

    match conn.close() {
        Ok(_) => { serde_json::to_string(&hv_info).unwrap_or_default() },
        Err(e) => format!("Failed to disconnect from hypervisor: code {}, message: {}",
        e.code,
        e.message),
    }
}

fn virt_show_domains() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(mut c) => {
            c
        },
        Err(e) => return format!("Not connected, code: {}, message: {}", e.code, e.message),
    };
    let mut hmir_domains:Vec<HmirDomain> = Vec::new();
    let flags = virt::connect::VIR_CONNECT_LIST_DOMAINS_ACTIVE;

    if let Ok(doms) = conn.list_all_domains(flags) {
        for dom in doms {
            let id = dom.get_id().unwrap_or(0);
            let name = dom.get_name().unwrap_or(String::from("no name"));
            if let Ok(dinfo) = dom.get_info(){
                hmir_domains.push(HmirDomain::new(id, name, dinfo.state, dinfo.max_mem, dinfo.nr_virt_cpu));
            }
        }
    }

    match conn.close() {
        Ok(_) => { serde_json::to_string(&hmir_domains).unwrap_or_default()},
        Err(e) => format!("Failed to disconnect from hypervisor: code {}, message: {}",
        e.code,
        e.message),
    }
}

fn virt_show_uri() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(mut c) => {
            c
        },
        Err(e) => return format!("Not connected, code: {}, message: {}", e.code, e.message),
    };

    let uri = match conn.get_uri(){
        Ok(s) => s,
        Err(e) => format!("Error, code:{}, message:{}", e.code, e.message),
    };

    match conn.close() {
        Ok(_) => { uri },
        Err(e) => format!("Failed to disconnect from hypervisor: code {}, message: {}",
        e.code,
        e.message),
    }
}
