//! 观察者模块
//!
//! 支持以下的请求
//! - reg-observer : 注册一个观察者
//!
//! 请求格式:
//! ```
//! {
//!    "jsonrpc":"2.0",
//!    "id":1,
//!    "method":"register-observer",
//!    "params":[$rest_url,$duration,$obs_type]
//! }
//! ```
//! - rest_url : 上传数据的endpoint，比如 https://172.30.24.123:5899/service_status
//! - duration : 通知频率，单位秒,最小1秒
//! - obs_type : 要观察的消息类型,当前支持以下类型:
//!
//! Ex:
//! ```
//! {
//!    "jsonrpc":"2.0",
//!    "id":1,
//!    "method":"register-observer",
//!    "params":["https://172.30.24.123:5899/service_status",1,0]
//! }
//! ```
//!

use jsonrpsee::ws_server::{RpcModule};
use std::sync::RwLock;
use hmir_hash::HashWrap;
use serde::{Deserialize};

#[cfg(target_os = "linux")]
use crate::svr::service_all;
use tokio::time;

const SERVICE_STATUS : u32 = 0;
use std::sync::{Arc};
use std::string;

macro_rules! observer_default_result {
    ($i:expr) =>{
        let mut response = HashWrap::<i32,i32>:: new();
        response.set_code($i);
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}

#[derive(Debug, Clone,Deserialize)]
struct ObserverParam {
    url : String, //now only support https
    obs_cmd : u32,
    duration : u64, //the frenquence
}

struct ObserverMetric {
    url : std::string::String, //now only support https
    obs_cmd : u32,
    duration : u64, //the frenquence
    status   : bool,
    callback: fn() -> String
}


type HandleType = Arc<RwLock<HashWrap<u32,ObserverMetric>>>;

lazy_static! {
    static ref OBSERVER_MAP : HandleType = {
        let m  = HashWrap::<u32,ObserverMetric>:: new();
        Arc::new(RwLock::new(m))
    };
}

pub fn do_nothing() -> std::string::String
{
    std::string::String::from("Ok")
}

type Callback = fn() -> std::string::String;


fn get_observer_callback(obs_cmd : u32) -> Callback {
    match obs_cmd {
        #[cfg(target_os = "linux")]
        SERVICE_STATUS => service_all,
        _ => do_nothing
    }
}

pub fn is_valid_obs_cmd(t : u32) -> bool {

    match t {
        SERVICE_STATUS => true,
        _ => false
    }
}


fn do_remote_post(result : & std::string::String , url : & std::string::String ){


}


fn create_obs_thread(map_handle : HandleType, obs_cmd : u32)
{
    tokio::task::spawn(async move {
        //运行在一个不阻塞的线程
        let mut count = 0;
        loop {
            if let Some(o) = map_handle.read().unwrap().get(&obs_cmd) {
                let url = o.url.clone();
                let duration = o.duration;
                let status = o.status;
                let call:Callback = o.callback;

                if !status {
                    map_handle.write().unwrap().remove(obs_cmd);
                    break
                }
                if count == duration {
                    let result = call();
                    do_remote_post(&result,&url);
                    count = 0;
                }
                std::thread::sleep(time::Duration::from_secs(1));
                count = count +1;
            }
        }
    });
}

fn reg_observer(obs_param : &ObserverParam) -> std::string::String
{
    if is_valid_obs_cmd(obs_param.obs_cmd) {

        //remove the old and stop the thread
        let metric = ObserverMetric {
            url : obs_param.url.clone(),
            obs_cmd: obs_param.obs_cmd,
            duration : obs_param.duration,
            callback : get_observer_callback(obs_param.obs_cmd),
            status : true
        };

        let obs_cmd = metric.obs_cmd;

        match  OBSERVER_MAP.read().unwrap().contains_key(&obs_cmd) {
            true => {
                if let Some(obs) = OBSERVER_MAP.write().unwrap().get_mut(&obs_cmd) {
                    //just upgrade the duration and url
                    obs.duration = metric.duration;
                    obs.url = metric.url;
                }
            }
            _ => {
                OBSERVER_MAP.write().unwrap().insert(obs_cmd,metric);
                let map_handle = OBSERVER_MAP.clone();
                create_obs_thread(map_handle,obs_cmd);
            }
        }

    }

    observer_default_result!(0);
}

pub fn unregister_observer(obs_cmd : u32) -> string::String
{
    if let Some(metric) = OBSERVER_MAP.write().unwrap().get_mut(&obs_cmd) {
        metric.status = false;
    }
    observer_default_result!(0);
}


pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>
{

    module.register_method("register-observer", |params,_| {
        //默认没有error就是成功的
        let obs_param = params.parse::<ObserverParam>()?;




        // let obs_type = obs_type.one::<u32>()?;
        // let duration = duration.one::<u32>()?;
        // println!("{}:{}:{}",url,obs_type,duration);
        // println!("{:?}",obs_param);
        Ok(reg_observer(&obs_param))
    })?;

    module.register_method("unregister-observer", |params, _| {
        //默认没有error就是成功的
        let obs_type = params.one::<u32>()?;

        Ok(unregister_observer(obs_type))
    })?;



    Ok(())

}