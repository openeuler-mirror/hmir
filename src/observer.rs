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
use std::{thread, time};
use serde::{Deserialize, Serialize};
use crate::svr::service_all;


enum obs_type {
    SERVICE_STATUS


}

#[derive(Debug, Clone,Deserialize)]
struct ObserverParam {
    url : std::string::String, //now only support https
    obs_type : u32,
    durition : u32, //the frenquence
}

struct ObserverMetric {
    url : std::string::String, //now only support https
    obs_type : u32,
    durition : u32, //the frenquence
    callback: fn() -> String
}


lazy_static! {
    static ref OBSERVER_MAP : RwLock<HashWrap<u32,ObserverMetric>> = {
        let m  = HashWrap::<u32,ObserverMetric>:: new();
        RwLock::new(m)
    };
}

pub fn do_nothing() -> std::string::String
{
    std::string::String::from("Ok")
}

type callback = fn() -> std::string::String;


fn get_observer_callback(obs_type : u32) -> callback {
    match obs_type {
        SERVICE_STATUS => service_all,
        _ => do_nothing
    }
}

pub fn is_valid_obs_type(obs_type : u32) -> bool {
    match obs_type {
        SERVICE_STATUS => true,
        _ => false
    }
}

fn reg_observer(obs_param : ObserverParam)
{

    if is_valid_obs_type(obs_param.obs_type) {
        let metric = ObserverMetric {
            url : obs_param.url,
            obs_type: obs_param.obs_type,
            durition : obs_param.durition,
            callback : get_observer_callback(obs_param.obs_type)
        };
        OBSERVER_MAP.write().unwrap().insert(metric.obs_type,metric);
    }

}

pub fn unregister_observer(obs_type : u32)
{
    if is_valid_obs_type(obs_type) {
        OBSERVER_MAP.write().unwrap().remove(obs_type);


    }


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
        Ok(reg_observer(obs_param))
    })?;

    module.register_method("unregister-observer", |params, _| {
        //默认没有error就是成功的
        let obs_type = params.one::<u32>()?;

        Ok(unregister_observer(obs_type))
    })?;

    Ok(())

}