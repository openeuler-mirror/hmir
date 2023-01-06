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
//! - virt-show-nwfilters: virt 展示nwfilter信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-nwfilters"
//! }
//!
//! - virt-show-libvirt-version: virt 展示libvirt version
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-libvirt-version"
//! }
//! 
//! - virt-show-arch-models: virt 展示cpu架构支持的models
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-arch-models"
//!     "params": {"arch":"x86_64"}
//! }
//! 

use  virt::connect::Connect;
use std::collections::HashMap;
use super::virt_type::*;

use jsonrpsee::ws_server::RpcModule;

const QEMU_URI: &str= "qemu:///system";

pub fn register_virt_query(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("virt-check-connection", |_, _| {
        Ok(virt_check_connection())
    })?;

    module.register_method("virt-show-hypervisor", |_, _| {
        Ok(virt_show_hypervisor())
    })?;

    module.register_method("virt-show-domains", |_, _| {
        Ok(virt_show_domains())
    })?;

    module.register_method("virt-show-uri", |_, _| {
        Ok(virt_show_uri())
    })?;

    module.register_method("virt-show-nwfilters", |_, _| {
        Ok(virt_show_nwfilters())
    })?;

    module.register_method("virt-show-libvirt-version", |_, _| {
        Ok(virt_show_libvirt_version())
    })?;

    module.register_method("virt-show-arch-models", |params, _| {
        let info = params.parse::<HashMap<String, String>>()?;
        Ok(virt_show_arch_models(info))
    })?;

    Ok(())
}

fn virt_check_connection() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
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

fn virt_show_hypervisor() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) => return format!("Not connected, code: {}, message: {}", e.code, e.message), 
    };

    let mut hv_info = HmirHvisor::default();

    if let Ok(hv_type) = conn.get_type() {
        if let Ok(mut hv_ver) = conn.get_hyp_version() {
            let hv_ver_str = translate_version(hv_ver);
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
        Ok(c) => {
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
        Ok(c) => {
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

fn virt_show_nwfilters() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) => return format!("Not connected, code: {}, message: {}", e.code, e.message),
    };

    let mut hmir_nwfilters:Vec<HmirNwfilter> = Vec::new();

    if let Ok(nwfilters) = conn.list_all_nw_filters(0) {
        for nw in nwfilters {
            let nw_name = nw.get_name().unwrap_or(String::from("no name"));
            let nw_uuid = nw.get_uuid_string().unwrap_or(String::from("no uuid"));
            hmir_nwfilters.push(HmirNwfilter::new(nw_name, nw_uuid));
        }
    }

    match conn.close() {
        Ok(_) => { serde_json::to_string(&hmir_nwfilters).unwrap_or_default() },
        Err(e) => format!("Failed to disconnect from hypervisor: code {}, message: {}",
        e.code,
        e.message),
    }
}

fn virt_show_libvirt_version() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) => return format!("Not connected, code: {}, message: {}", e.code, e.message),
    };

    let mut lib_ver = match conn.get_lib_version(){
        Ok(ver) => translate_version(ver),
        Err(e) =>  format!("Not connected, code: {}, message: {}", e.code, e.message),
    };

    match conn.close() {
        Ok(_) => { lib_ver },
        Err(e) => format!("Failed to disconnect from hypervisor: code {}, message: {}",
        e.code,
        e.message),
    }
}

fn translate_version(mut ver: u32) -> String{
    let major = ver / 1000000;
    ver %= 1000000;
    let minor = ver / 1000;
    let release = ver % 1000;
    format!("{}.{}.{}", major, minor, release)
}

fn virt_show_arch_models(info: HashMap<String, String>) -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) => return format!("Not connected, code: {}, message: {}", e.code, e.message), 
    };

    let arch_name = info.get("arch").unwrap();

    let models = conn.get_cpu_models_names(&arch_name, 0).unwrap();

    match conn.close() {
        Ok(_) => { serde_json::to_string(&models).unwrap_or_default() },
        Err(e) => format!("Failed to disconnect from hypervisor: code {}, message: {}",
        e.code,
        e.message),
    }
}