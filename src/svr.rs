//! 服务管理模块
//!
//! 支持以下的请求
//! - service-status : 查询指定服务状态
//! - service-all    : 所有服务状态
//! - service-disable : disable服务
//! - service-enable : enable服务
//! - service-stop   : stop服务
//! - service-restart : restart服务
//! - service-start : start服务
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
//!     "code":0
//!     "result":{
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
use std::sync::{Mutex};
use hmir_hash::HashWrap;
use hmir_errno::errno;
// use std::{thread, time};
use tokio::time::{sleep, Duration};

use std::path::Path;
use std::ffi::OsStr;

use hmir_systemd::{build_blocking_client,
                   unit::blocking::UnitProperties,manager::blocking::{OrgFreedesktopSystemd1Manager},
                   models::{Unit,HmirUnit, UnitProps,IntoModel}, SystemdObjectType};
use hmir_token::TokenChecker;



lazy_static! {
    // static ref SERVICE_MAP : RwLock<HashWrap<String,Unit>> = {
    //     let m  = HashWrap::<String,Unit>:: new();
    //     RwLock::new(m)
    // };

    static ref SERVICE_ENABLED_CACHE: Mutex<String> = Mutex::new(String::new());
    static ref SERVICE_DISABLED_CACHE : Mutex<String> = Mutex::new(String::new());
    static ref SERVICE_STATIC_CACHE : Mutex<String> = Mutex::new(String::new());

    static ref TIMER_ENABLED_CACHE: Mutex<String> = Mutex::new(String::new());
    static ref TIMER_DISABLED_CACHE : Mutex<String> = Mutex::new(String::new());
    static ref TIMER_STATIC_CACHE : Mutex<String> = Mutex::new(String::new());


    static ref SOCKET_ENABLED_CACHE: Mutex<String> = Mutex::new(String::new());
    static ref SOCKET_DISABLED_CACHE : Mutex<String> = Mutex::new(String::new());
    static ref SOCKET_STATIC_CACHE : Mutex<String> = Mutex::new(String::new());



    static ref TARGET_ENABLED_CACHE: Mutex<String> = Mutex::new(String::new());
    static ref TARGET_DISABLED_CACHE : Mutex<String> = Mutex::new(String::new());
    static ref TARGET_STATIC_CACHE : Mutex<String> = Mutex::new(String::new());

    static ref SLICE_ALL_CACHE  : Mutex<String> = Mutex::new(String::new());
}

macro_rules! svr_default_result {
    ( $result : expr ) =>{
        let mut response = HashWrap::<String,Unit>:: new();
        match $result {
            Ok(_x) => { response.set_code(0); },
            Err(_e) => { response.set_code(errno::HMIR_ERR_COMM); },
        }
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}


#[doc(hidden)]
fn update_all_svr()
{
    *SERVICE_ENABLED_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["enabled"],vec!["*.service"]);
    *SERVICE_DISABLED_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["disabled"],vec!["*.service"]);
    *SERVICE_STATIC_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["static"],vec!["*.service"]);


    *SOCKET_ENABLED_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["enabled"],vec!["*.socket"]);
    *SOCKET_DISABLED_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["disabled"],vec!["*.socket"]);
    *SOCKET_STATIC_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["static"],vec!["*.socket"]);


    *TIMER_ENABLED_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["enabled"],vec!["*.timer"]);
    *TIMER_DISABLED_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["disabled"],vec!["*.timer"]);
    *TIMER_STATIC_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["static"],vec!["*.timer"]);


    *TARGET_ENABLED_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["enabled"],vec!["*.target"]);
    *TARGET_DISABLED_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["disabled"],vec!["*.target"]);
    *TARGET_STATIC_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["static"],vec!["*.target"]);

    *SLICE_ALL_CACHE.lock().unwrap() = get_unit_list_by_pattern(vec!["static","disabled","enabled"],vec!["*.slice"]);

}

fn get_unit_list_by_pattern(states: Vec<&str>, patterns: Vec<&str>) -> String {
    let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    let files = client.list_unit_files_by_patterns(states,patterns).unwrap();
    // println!("{:?}",files);
    let vec_files:Vec<String> = files.into_iter().map(|n| n.0 ).collect::<_>();

    let vec_filenames: Vec<&str> = vec_files.iter().map(|n| basename(n.as_ref()).unwrap()).collect();
    let mut map  = hmir_hash::HashWrap::new();


    for x in &vec_filenames {
        match client.list_units_by_names(vec![x]) {
            Ok(units) => {
                for u in units {

                    let client = build_blocking_client(SystemdObjectType::Unit(u.6.clone())).unwrap();
                    let props_map = client.get_unit_properties().unwrap();
                    let unit_props: UnitProps = props_map.into_model().unwrap();
                    let unit: Unit = u.into_model().unwrap().clone();
                    let hmir_unit = HmirUnit {
                        name: unit.name.into(),
                        description: unit.description.into(),
                        load_state: unit.load_state.into(),
                        active_state: unit.active_state.into(),
                        sub_state: unit.sub_state.into(),
                        follow_unit: unit.follow_unit.into(),
                        object_path: unit.object_path.into(),
                        job_id: unit.job_id,
                        job_ty: unit.job_ty,
                        job_object_path: unit.job_object_path,
                        requires: unit_props.requires.into(),
                        wants : unit_props.wants.into(),
                        wantedby: unit_props.wantedby.into(),
                        conflicts: unit_props.conflicts.into(),
                        before: unit_props.before.into(),
                        after: unit_props.after.into(),
                    };

                    // println!("{:?}",hmir_unit);
                    map.insert(hmir_unit.name.clone(),hmir_unit.clone());
                }
            }
            _ => {}
        }
    }
    let serialized = serde_json::to_string(&map).unwrap();
    serialized
}


#[doc(hidden)]
pub fn init_services_mg()  {
    tokio::task::spawn(async{
        //运行在一个不阻塞的线程
        let sec = Duration::from_secs(1);
        loop {
            update_all_svr();
            sleep(sec).await;
        }
    });
}

fn basename(filename: &str) -> Option<&str> {
    let name = Path::new(filename).file_name().and_then(OsStr::to_str);

    // println!("{:?}",name);
    name
}


///
/// service-all接口实现
///
/// 获取所有服务信息
pub fn svr_list_enabled_service() -> String {
    let result = (*SERVICE_ENABLED_CACHE.lock().unwrap()).clone();
    result
}

pub fn svr_list_disabled_service() -> String {
    let result = (*SERVICE_DISABLED_CACHE.lock().unwrap()).clone();
    result
}

pub fn svr_list_static_service() -> String {
    let result = (*SERVICE_STATIC_CACHE.lock().unwrap()).clone();
    result
}

pub fn svr_list_enabled_timer() -> String {
    let result = (*TIMER_ENABLED_CACHE.lock().unwrap()).clone();
    result
}

pub fn svr_list_disabled_timer() -> String {
    let result = (*TIMER_DISABLED_CACHE.lock().unwrap()).clone();
    result
}

pub fn svr_list_static_timer() -> String {
    let result = (*TIMER_STATIC_CACHE.lock().unwrap()).clone();
    result
}

pub fn svr_list_enabled_socket() -> String {
    let result = (*SOCKET_ENABLED_CACHE.lock().unwrap()).clone();
    result
}

pub fn svr_list_disabled_socket() -> String {
    let result = (*SOCKET_DISABLED_CACHE.lock().unwrap()).clone();
    result}

pub fn svr_list_static_socket() -> String {
    let result = (*SOCKET_STATIC_CACHE.lock().unwrap()).clone();
    result
}

pub fn svr_list_enabled_target() -> String {
    let result = (*TARGET_ENABLED_CACHE.lock().unwrap()).clone();
    result
}

pub fn svr_list_disabled_target() -> String {
    let result = (*TARGET_DISABLED_CACHE.lock().unwrap()).clone();
    result}

pub fn svr_list_static_target() -> String {
    let result = (*TARGET_STATIC_CACHE.lock().unwrap()).clone();
    result
}

pub fn svr_list_all_slice() -> String {
    let result = (*SLICE_ALL_CACHE.lock().unwrap()).clone();
    result
}

///
/// service-stop接口实现
///
/// 停止指定服务
pub fn service_stop(service: String) -> String {
    // let mut  response = HashWrap::<String,Unit>:: new();
    let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    let result = client.stop_unit(service.as_str(), "replace");
    svr_default_result!(result);
}


///
/// service-start接口实现
///
/// 启动指定服务
pub fn service_start(service: String) -> String {
    let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    let result = client.start_unit(service.as_str(), "replace");
    svr_default_result!(result);
}


///
/// service-restart接口实现
///
/// 重启指定服务
pub fn service_restart(service: std::string::String) -> String {
    let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    let result = client.restart_unit(service.as_str(), "replace");
    svr_default_result!(result);
}

///
/// service-disable接口实现
///
/// disable指定服务
pub fn service_disable(service: std::string::String) -> String {
    let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    let result = client.disable_unit_files(vec![service.as_str()], true);
    svr_default_result!(result);
}


///
/// service-enable接口实现
///
/// enable指定服务
pub fn service_enable(service: std::string::String) -> String {
    let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
    let result = client.enable_unit_files(vec![service.as_str()], false,true);
    svr_default_result!(result);
}




pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    //The svr module
    module.register_method("svr-list-enabled-service", |params, _| {
        //默认没有error就是成功的
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);
        Ok(svr_list_enabled_service())
    })?;

    module.register_method("svr-list-disabled-service", |params, _| {
        //默认没有error就是成功的
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);

        Ok(svr_list_disabled_service())
    })?;

    module.register_method("svr-list-static-service", |params, _| {
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);

        //默认没有error就是成功的
        Ok(svr_list_static_service())
    })?;


    //The time api
    module.register_method("svr-list-enabled-timer", |params, _| {
        //默认没有error就是成功的

        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);

        Ok(svr_list_enabled_timer())
    })?;

    module.register_method("svr-list-disabled-timer", |params, _| {
        //默认没有error就是成功的
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);

        Ok(svr_list_disabled_timer())
    })?;

    module.register_method("svr-list-static-timer", |params, _| {
        //默认没有error就是成功的

        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);

        Ok(svr_list_static_timer())
    })?;


    //The time api
    module.register_method("svr-list-enabled-socket", |params, _| {
        //默认没有error就是成功的

        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);

        Ok(svr_list_enabled_socket())
    })?;

    module.register_method("svr-list-disabled-socket", |params, _| {
        //默认没有error就是成功的

        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);

        Ok(svr_list_disabled_socket())
    })?;

    module.register_method("svr-list-static-socket", |params, _| {

        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);

        //默认没有error就是成功的
        Ok(svr_list_static_socket())
    })?;


    //The time api
    module.register_method("svr-list-enabled-target", |params, _| {
        //默认没有error就是成功的

        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);

        Ok(svr_list_enabled_target())
    })?;

    module.register_method("svr-list-disabled-target", |params, _| {
        //默认没有error就是成功的

        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);
        Ok(svr_list_disabled_target())
    })?;

    module.register_method("svr-list-static-target", |params, _| {

        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);

        //默认没有error就是成功的
        Ok(svr_list_static_target())
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


    module.register_method("svr-list-all-slice", |params, _| {
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);

        //默认没有error就是成功的
        Ok(svr_list_all_slice())
    })?;


    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn svr_result_test() -> std::string::String {
        let client = build_blocking_client(SystemdObjectType::Manager).unwrap();
        let vec = vec!["docker.service"];
        let result = client.enable_unit_files(vec, false,true);
        svr_default_result!(result);
    }

    #[test]
    fn svr_result_it_work()  {

        let s = svr_result_test();
        println!("{}",s);
    }

    #[test]
    fn svr_list_enabled_service_worked() {
        update_all_svr();
        let data = svr_list_enabled_service();
        println!("{}",data);
    }

    #[test]
    fn service_restart_it_works() {
        let s = service_restart(std::string::String::from("docker.service"));
        println!("{}",s);
    }


    #[test]
    fn service_stop_it_works() {
        let s = service_stop(std::string::String::from("docker.service"));
        println!("{}",s);
    }

    #[test]
    fn service_start_it_works() {
        let s = service_start(std::string::String::from("docker.service"));
        println!("{}",s);
    }

    #[test]
    fn service_disable_it_works(){
        let s = service_disable(std::string::String::from("docker.service"));
        println!("{}",s);
    }

    #[test]
    fn service_list_disable_it_works(){
        let s = svr_list_disabled_service();
        println!("{}",s);
    }

    #[test]
    fn basename_test(){
        basename("/lib/systemd/system/drbd@.target");
    }
}