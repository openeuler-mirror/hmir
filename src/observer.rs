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
use serde::{Deserialize, Serialize};
use crate::svr::service_all;
use tokio::time;

const SERVICE_STATUS : u32 = 0;
use std::sync::{Arc, Mutex};
use std::string;

#[derive(Debug, Clone,Deserialize)]
struct ObserverParam {
    url : std::string::String, //now only support https
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


lazy_static! {
    static ref OBSERVER_MAP : Arc<RwLock<HashWrap<u32,ObserverMetric>>> = {
        let m  = HashWrap::<u32,ObserverMetric>:: new();
        Arc::new(RwLock::new(m))
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

pub fn is_valid_obs_cmd(t : u32) -> bool {

    match t {
        SERVICE_STATUS => true,
        _ => false
    }
}


fn do_remote_post(result : & std::string::String , url : & std::string::String ){


    

}


fn reg_observer(obs_param : &ObserverParam) -> std::string::String
{
    if is_valid_obs_cmd(obs_param.obs_cmd) {

        //remove the old and stop the thread
        let mut metric = ObserverMetric {
            url : obs_param.url.clone(),
            obs_cmd: obs_param.obs_cmd,
            duration : obs_param.duration,
            callback : get_observer_callback(obs_param.obs_cmd),
            status : true
        };

        let call:callback = metric.callback;
        let obs_cmd = metric.obs_cmd;


        if OBSERVER_MAP.write().unwrap().contains_key(&obs_param.obs_cmd) {
            //update the value
            metric.status = OBSERVER_MAP.read().unwrap().get(&obs_cmd).unwrap().status;
            OBSERVER_MAP.write().unwrap().remove(metric.obs_cmd);
            OBSERVER_MAP.write().unwrap().insert(metric.obs_cmd,metric);

        } else {
            OBSERVER_MAP.write().unwrap().insert(metric.obs_cmd,metric);
            let map_handle = OBSERVER_MAP.clone();

            tokio::task::spawn(async move {
                //运行在一个不阻塞的线程
                loop {
                    let url = map_handle.read().unwrap().get(&obs_cmd).unwrap().url.clone();
                    let duration = map_handle.read().unwrap().get(&obs_cmd).unwrap().duration;
                    let mut count = 0;
                    if count == duration {
                        let result = call();
                        do_remote_post(&result,&url);
                    }

                    let status = map_handle.read().unwrap().get(&obs_cmd).unwrap().status;
                    if !status {
                        OBSERVER_MAP.write().unwrap().remove(obs_cmd);
                        break
                    }

                    tokio::time::sleep(time::Duration::from_secs(1));
                    count = count +1;
                }
            });
        }
    }
    return string::String::from("Ok");
}



pub fn unregister_observer(obs_cmd : u32)
{
    if OBSERVER_MAP.write().unwrap().contains_key(&obs_cmd) {
        if let Some(metric) = OBSERVER_MAP.write().unwrap().get_mut(&obs_cmd) {
            metric.status = false;
        }
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
        Ok(reg_observer(&obs_param))
    })?;

    module.register_method("unregister-observer", |params, _| {
        //默认没有error就是成功的
        let obs_type = params.one::<u32>()?;

        Ok(unregister_observer(obs_type))
    })?;



    Ok(())

}