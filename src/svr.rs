//! 服务管理模块
//!
//! 支持以下的请求
//! - service-status : 查询指定服务状态
//!
//! 请求格式:
//! ```
//! {
//!    "jsonrpc":"2.0",
//!    "id":1,
//!    "method":"service-status",
//!    "params":["collectl.service"]
//! }
//! ```
//! 响应格式:
//!
//! ```
//! {
//!      "jsonrpc": "2.0",
//!      "result": "{\"map\":{\"collectl.service\":{\"name\":\"collectl.service\",\"description\":\"LSB: Collectl monitors system performance.\",\"load_state\":\"Loaded\",\"active_state\":\"Inactive\",\"sub_state\":\"Dead\",\"follow_unit\":null,\"object_path\":\"/org/freedesktop/systemd1/unit/collectl_2eservice\",\"job_id\":0,\"job_ty\":\"\",\"job_object_path\":\"/\"}}}",
//!      "id": 1
//! }
//! ```
//! result字段以字符串的形式存放了返回的json结果。
//! 下面是result字符串以json格式化的结果:
//! ```
//! {
//!     "map":{
//!         "collectl.service":{
//!             "name":"collectl.service",
//!             "description":"LSB: Collectl monitors system performance.",
//!             "load_state":"Loaded",
//!             "active_state":"Inactive",
//!             "sub_state":"Dead",
//!             "follow_unit":null,
//!             "object_path":"/org/freedesktop/systemd1/unit/collectl_2eservice",
//!             "job_id":0,
//!             "job_ty":"",
//!             "job_object_path":"/"
//!         }
//!     }
//! }
//!```
//!  - service-all : 查询所有服务状态
//!
//! 请求格式:
//! ```
//! {
//!    "jsonrpc":"2.0",
//!    "id":1,
//!    "method":"service-all"
//! }
//! ```
//!  - service-stop : 停止指定服务
//!
//! 请求格式:
//! ```
//! {
//!    "jsonrpc":"2.0",
//!    "id":1,
//!    "method":"service-stop",
//!    "params":["collectl.service"]
//! }
//! ```
//!  - service-start : 启动指定服务
//!
//! 请求格式:
//! ```
//! {
//!    "jsonrpc":"2.0",
//!    "id":1,
//!    "method":"service-start",
//!    "params":["collectl.service"]
//! }
//! ```

use jsonrpsee::ws_server::{RpcModule};
use std::sync::RwLock;
use hmir_hash::HashWrap;
use std::{thread, time};


use hmir_systemd::{
    build_blocking_client,
    manager::blocking::{OrgFreedesktopSystemd1Manager},
    models::{Unit,IntoModel},
    SystemdObjectType,
};


lazy_static! {
    static ref SERVICE_MAP : RwLock<HashWrap<String,Unit>> = {
        let m  = HashWrap::<String,Unit>:: new();
        RwLock::new(m)
    };
}


#[doc(hidden)]
fn update_all_svr()
{
    let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    let units = client.list_units().unwrap();
    for unit in units {
        let unit: Unit = unit.into_model().unwrap().clone();
        SERVICE_MAP.write().unwrap().insert(unit.name.clone(),unit.clone());
    }
}

#[doc(hidden)]
fn create_svrmg_thread()
{
    tokio::task::spawn(async{
        //运行在一个不阻塞的线程
        let sec = time::Duration::from_millis(1000);
        loop {
            update_all_svr();
            thread::sleep(sec);
        }
    });
}


#[doc(hidden)]
pub fn init_services_mg()  {
    create_svrmg_thread();
}


///
/// service-all接口实现
///
/// 获取所有服务信息
pub fn service_all() -> String {
    let serialized = serde_json::to_string(&*SERVICE_MAP).unwrap();
    serialized
}

///
/// service-status接口实现
///
/// 获取指定服务信息
pub fn service_status(service: std::string::String) -> String
{
    let mut  result = HashWrap::new();
    if SERVICE_MAP.read().unwrap().contains_key(&service) {
        let value = SERVICE_MAP.read().unwrap().get(&service).unwrap().clone();
        result.insert(service,value);
    }
    let serialized = serde_json::to_string(&result).unwrap();
    serialized

}

///
/// service-stop接口实现
///
/// 停止指定服务
pub fn service_stop(service: std::string::String) -> String {
    let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    let result = client.stop_unit(service.as_str(), "replace");
    match result {
        Ok(path) => String::from("Ok"),
        Err(e) => String::from(e.name().unwrap())
    }
}


///
/// service-start接口实现
///
/// 启动指定服务
pub fn service_start(service: std::string::String) -> String {
    let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    let result = client.start_unit(service.as_str(), "replace");
    match result {
        Ok(path) => String::from("Ok"),
        Err(e) => String::from(e.name().unwrap())
    }
}


///
/// service-restart接口实现
///
/// 重启指定服务
pub fn service_restart(service: std::string::String) -> String {
    let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    let result = client.restart_unit(service.as_str(), "replace");
    match result {
        Ok(path) => String::from("Ok"),
        Err(e) => String::from(e.name().unwrap())
    }
}

///
/// service-disable接口实现
///
/// disable指定服务
pub fn service_disable(service: std::string::String) -> String {
    // let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    // let result = client.disable_unit(service.as_str(), "replace");
    // match result {
    //     Ok(path) => String::from("Ok"),
    //     Err(e) => String::from(e.name().unwrap())
    // }
    todo!();
}


///
/// service-enable接口实现
///
/// enable指定服务
pub fn service_enable(service: std::string::String) -> String {
    // let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    // let result = client.enable_unit(service.as_str(), "replace");
    // match result {
    //     Ok(path) => String::from("Ok"),
    //     Err(e) => String::from(e.name().unwrap())
    // }
    todo!();
}


pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    //The svr module

    module.register_method("service-all", |_, _| {
        //默认没有error就是成功的
        Ok(service_all())
    })?;


    module.register_method("service-status", |params, _| {
        let service = params.one::<std::string::String>()?;
        Ok(service_status(service))
    })?;


    module.register_method("service-start", |params, _| {
        //默认没有error就是成功的
        let service = params.one::<std::string::String>()?;
        Ok(service_start(service))
    })?;

    module.register_method("service-stop", |params, _| {
        //默认没有error就是成功的
        let service = params.one::<std::string::String>()?;
        Ok(service_stop(service))
    })?;

    module.register_method("service-restart", |params, _| {
        //默认没有error就是成功的
        let service = params.one::<std::string::String>()?;
        Ok(service_restart(service))
    })?;

    module.register_method("service-disable", |params, _| {
        //默认没有error就是成功的
        let service = params.one::<std::string::String>()?;
        Ok(service_disable(service))
    })?;

    module.register_method("service-enable", |params, _| {
        //默认没有error就是成功的
        let service = params.one::<std::string::String>()?;
        Ok(service_enable(service))
    })?;

    Ok(())
}