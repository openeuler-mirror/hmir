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
//! - virt-show-domains: virt 展示所有 domains信息
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
//!     "method":"virt-show-arch-models",
//!     "params": {"arch":"x86_64"}
//! }
//! 
//! - virt-show-networks: virt 展示网络信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-networks"
//! }
//! 
//! - virt-show-interfaces: virt 展示网络接口信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-interfaces"
//! }
//! 
//! - virt-show-secrets: virt 展示加密管理信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-secrets"
//! }
//! 
//! - virt-show-storagepools: virt 展示存储池信息
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-storagepools"
//! }
//! 
//! - virt-show-nodedevs: virt 展示节点所有设备名称
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-nodedevs"
//! }
//! 
//! - virt-show-sys-info: virt 展示 sys info 说明
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-show-sys-info"
//! }
//!


use  virt::connect::Connect;
use super::virt_type::*;

use std::{collections::BTreeMap}; 
use serde_json::{json, Value};
use jsonrpsee::ws_server::RpcModule;

const QEMU_URI: &str= "qemu:///system";

use hmir_errno::errno;
use hmir_hash::HashWrap;
use hmir_token::TokenChecker;

use std::sync::{Arc,Mutex};
use lazy_static::{lazy_static};

struct WrapperCoon(Connect);
unsafe impl Send for WrapperCoon{}
unsafe impl Sync for WrapperCoon{}

type HyperStaticConn = Arc<Mutex<Option<WrapperCoon>>>;

lazy_static!{
    static ref QEMU_CONN: HyperStaticConn = {
        match Connect::open(QEMU_URI){
            Ok(c) => {
                let p = Some(WrapperCoon(c));
                return Arc::new(Mutex::new(p));
            },
            Err(_) => return Arc::new(Mutex::new(None))
        }
    };
}


macro_rules! VirtTokenChecker {
    ($info:expr) => {
        let token_exception = json!(String::from(""));
        let token = ($info).get("token").unwrap_or(&token_exception).to_string();
        TokenChecker!(token);
    }
}

macro_rules! ExecVirtQueryResult {
    ($i:expr, $j:expr, $k:expr) => {
        let mut response = HashWrap::<String,String>:: new();
        if $i == 0 {
            response.insert(String::from("virt_ret"), $j);
        }

        if $i !=0 {
            response.error($i, $k);
        } 
        response.set_code($i);
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}

pub fn register_virt_query(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("virt-check-connection", |params, _| {
        //let info = params.parse::<BTreeMap<&str, Value>>()?;
        //VirtTokenChecker!(info);
        Ok(virt_check_connection())
    })?;

    module.register_method("virt-show-hypervisor", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_hypervisor())
    })?;

    module.register_method("virt-show-domains", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_domains())
    })?;

    module.register_method("virt-show-uri", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_uri())
    })?;

    module.register_method("virt-show-nwfilters", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_nwfilters())
    })?;

    module.register_method("virt-show-libvirt-version", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_libvirt_version())
    })?;

    module.register_method("virt-show-arch-models", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_arch_models(info))
    })?;

    module.register_method("virt-show-networks", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_networks())
    })?;

    module.register_method("virt-show-interfaces", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_interfaces())
    })?;

    module.register_method("virt-show-secrets", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_secrets())
    })?;

    module.register_method("virt-show-storagepools", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_storagepools())
    })?;

    module.register_method("virt-show-nodedevs", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_nodedevs())
    })?;

    module.register_method("virt-show-sys-info", |params, _| {
        let info = params.parse::<BTreeMap<&str, Value>>()?;
        VirtTokenChecker!(info);
        Ok(virt_show_sys_info())
    })?;

    Ok(())
}

fn virt_check_connection() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    };
    
    match conn.close() {
        Ok(_) => {
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, "Connected!".to_string(), "".to_string());
        },
        Err(e) => {
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        }
    }
}

fn virt_show_hypervisor() -> String{

    let conn_ref = QEMU_CONN.lock().unwrap();
    let conn = match conn_ref.as_ref(){
        Some(c) => &c.0,
        None =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), "Not Connected!".to_string());
        },
    };

    let mut hv_info = HmirHvisor::default();
    if let Ok(hv_type) = conn.get_type() {
        if let Ok(hv_ver) = conn.get_hyp_version() {
            let hv_ver_str = translate_version(hv_ver);
            let is_alive = conn.is_alive().unwrap_or_default();
            let is_enc = conn.is_encrypted().unwrap_or_default();
            let is_sec= conn.is_secure().unwrap_or_default();
            hv_info = HmirHvisor::new(hv_type, hv_ver_str, is_alive, is_enc, is_sec);
            
            let ret_info = serde_json::to_string(&hv_info).unwrap_or_default();
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, ret_info, "".to_string());
        }
    }
    
    ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), "Failed to show hypervisor".to_string());
}

fn virt_show_domains() -> String{
    let conn_ref = QEMU_CONN.lock().unwrap();
    let conn = match conn_ref.as_ref(){
        Some(c) => &c.0,
        None =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), "Not Connected!".to_string());
        },
    };

    let mut hmir_domains:Vec<HmirDomain> = Vec::new();
    let flags = virt::connect::VIR_CONNECT_LIST_DOMAINS_ACTIVE |
                virt::connect::VIR_CONNECT_LIST_DOMAINS_INACTIVE;

    if let Ok(doms) = conn.list_all_domains(flags) {
        for dom in doms {
            let id = dom.get_id().unwrap_or(0);
            let name = dom.get_name().unwrap_or(String::from("no name"));
            let uuid = dom.get_uuid_string().unwrap_or_default();
            hmir_domains.push(HmirDomain::new(id, name, uuid));
        }
        let ret_info = serde_json::to_string(&hmir_domains).unwrap_or_default();
        ExecVirtQueryResult!(errno::HMIR_SUCCESS, ret_info, "".to_string());
    }

    ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), "Failed to show domains".to_string());
}

fn virt_show_uri() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    };

    let uri = match conn.get_uri(){
        Ok(s) => s,
        Err(e) =>  format!("Failed to get uri, code: {}, message: {}", e.code, e.message),
    };

    match conn.close() {
        Ok(_) => { 
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, uri, "".to_string());
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    }
}

fn virt_show_nwfilters() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
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
        Ok(_) => { 
            let ret_info = serde_json::to_string(&hmir_nwfilters).unwrap_or_default(); 
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, ret_info, "".to_string());
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    }
}

fn virt_show_libvirt_version() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    };

    let lib_ver = match conn.get_lib_version(){
        Ok(ver) => translate_version(ver),
        Err(e) =>  format!("Failed to get lib version, code: {}, message: {}", e.code, e.message),
    };

    match conn.close() {
        Ok(_) => { 
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, lib_ver, "".to_string());
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    }
}

fn translate_version(mut ver: u32) -> String{
    let major = ver / 1000000;
    ver %= 1000000;
    let minor = ver / 1000;
    let release = ver % 1000;
    format!("{}.{}.{}", major, minor, release)
}

fn virt_show_arch_models(info: BTreeMap<&str, Value>) -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    };

    let arch_name = info.get("arch").unwrap().to_string();
    let models = conn.get_cpu_models_names(&arch_name, 0).unwrap();

    match conn.close() {
        Ok(_) => { 
            let ret_info = serde_json::to_string(&models).unwrap_or_default(); 
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, ret_info, "".to_string());
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    }
}

fn virt_show_networks() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    };

    let mut vec_net: Vec<HmirNetwork> = Vec::new();
    if let Ok(virt_networks) =  conn.list_all_networks(0){
        for net in virt_networks{
            let name = net.get_name().unwrap_or(String::from("no name"));
            let uuid = net.get_uuid_string().unwrap_or(String::from("no uuid"));
            let bridge = net.get_bridge_name().unwrap_or(String::from("no bridge"));
            let is_active = net.is_active().unwrap_or_default();
            let is_persist = net.is_persistent().unwrap_or_default();
            vec_net.push(HmirNetwork::new(name, uuid, bridge, is_active, is_persist));
        }
    }

    match conn.close() {
        Ok(_) => { 
            let ret_info = serde_json::to_string(&vec_net).unwrap_or_default(); 
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, ret_info, "".to_string());
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    }

}

fn virt_show_interfaces() ->String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    };

    let mut vec_if: Vec<HmirInterface> = Vec::new();
    if let Ok(interfaces) = conn.list_all_interfaces(0) {
        for interface in interfaces{
            let name = interface.get_name().unwrap_or_default();
            let mac = interface.get_mac_string().unwrap_or_default();
            let is_active = interface.is_active().unwrap_or_default();
            vec_if.push(HmirInterface::new(name, mac, is_active));
        }
    }
    
    match conn.close() {
        Ok(_) => { 
            let ret_info = serde_json::to_string(&vec_if).unwrap_or_default(); 
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, ret_info, "".to_string());
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    }
}

fn virt_show_secrets() -> String {
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    };

    let mut vec_secs: Vec<HmirSecret> = Vec::new();
    if let Ok(secrets) = conn.list_all_secrets(0){
        for sec in secrets {
            let uuid = sec.get_uuid_string().unwrap_or_default();
            let usage = sec.get_usage_id().unwrap_or_default();
            let usage_id = sec.get_usage_type().unwrap_or_default();
            vec_secs.push(HmirSecret::new(uuid, usage, usage_id));
        }
    }

    match conn.close() {
        Ok(_) => { 
            let ret_info = serde_json::to_string(&vec_secs).unwrap_or_default(); 
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, ret_info, "".to_string());
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    }
}

fn virt_show_storagepools() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    };

    let mut vec_sps:Vec<HmirStoragePool> = Vec::new();

    if let Ok(sps) = conn.list_all_storage_pools(0){
        for sp in sps{
            let uuid = sp.get_uuid_string().unwrap_or_default();
            if let Ok(sp_info) = sp.get_info(){
                vec_sps.push(HmirStoragePool::new(uuid, sp_info.state, 
                    sp_info.capacity, sp_info.allocation, sp_info.available));
            }
        }
    }

    match conn.close() {
        Ok(_) => { 
            let ret_info = serde_json::to_string(&vec_sps).unwrap_or_default(); 
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, ret_info, "".to_string());
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    }
}

fn virt_show_nodedevs() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    };

    let mut vec_caps : Vec<String> = Vec::new();
    if let Ok(node_devs) = conn.list_all_node_devices(0){
        for dev in node_devs{
            if let Ok(caps) = dev.list_caps(){
                for cap in caps {
                    vec_caps.push(cap);
                }
            }
        }
    }

    match conn.close() {
        Ok(_) => { 
            let ret_info = serde_json::to_string(&vec_caps).unwrap_or_default(); 
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, ret_info, "".to_string());
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    }

}

fn virt_show_sys_info() -> String{
    let mut conn = match Connect::open(QEMU_URI) {
        Ok(c) => {
            c
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    };

    let sys_info = conn.get_sys_info(0).unwrap_or_default();

    match conn.close() {
        Ok(_) => { 
            ExecVirtQueryResult!(errno::HMIR_SUCCESS, sys_info, "".to_string());
        },
        Err(e) =>{
            ExecVirtQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), format!("code: {}, error: {}", e.code, e.message));
        } 
    }
}