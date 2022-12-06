//! 进程管理模块
//!
//!


use jsonrpsee::ws_server::{RpcModule};

use procfs;
use serde::{Deserialize, Serialize};
use hmir_hash::HashWrap;


#[derive(Clone, Debug,Serialize)]
struct ProcInfo {
    pub pid: i32,
    pub comm: String
}

pub fn process_all() -> std::string::String
{
    let mut map = HashWrap::<i32,ProcInfo>:: new();
    for prc in procfs::process::all_processes().unwrap() {
        // println!("{:?}",prc);
        if let Ok(stat) = prc.unwrap().stat() {
            // total_time is in seconds
            let p  = ProcInfo {
                pid: stat.pid,
                comm: stat.comm
            };
            map.insert(stat.pid,p);
        }
    }

    let serialized = serde_json::to_string(&map).unwrap();
    serialized
}

pub fn process_status() -> std::string::String {


    todo!()

}

pub fn process_kill() -> std::string::String {
    todo!()

}


#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("process-all", |_, _| {
        //默认没有error就是成功的
        Ok(process_all())
    })?;

    Ok(())
}