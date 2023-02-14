//! 进程管理模块
//!
//!
//! 支持以下的请求
//! - process-all : 查询指定服务状态
//!
//! 请求格式:
//! ```
//! {
//!    "jsonrpc":"2.0",
//!    "id":1,
//!    "method":"process-all"
//! }
//! ```

use jsonrpsee::ws_server::{RpcModule};

use procfs;
use serde::{Serialize};
use hmir_hash::HashWrap;
use hmir_errno::errno;
use nix::sys::signal;
use nix::unistd;
use nix::libc;
use std::ffi::CStr;
use hmir_protocol::proc;

extern crate core_affinity;

macro_rules! proc_default_result {
    ($i:expr) =>{
        let mut response = HashWrap::<i32,proc::ProcInfo>:: new();
        response.set_code($i);
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}

fn proc_get_username(uid : libc::uid_t) -> String
{
    unsafe {
        let passwd = nix::libc::getpwuid(uid);
        let username = CStr::from_ptr((*passwd).pw_name);
        String::from(username.to_str().unwrap())
    }
}

fn proc_get_pagesize() -> i64 {
    unsafe {
        libc::sysconf(libc::_SC_PAGESIZE)
    }
}

pub fn process_all() -> std::string::String
{
    let mut map = HashWrap::<i32,proc::ProcInfo>:: new();
    let page_sizeKB = proc_get_pagesize() as u64 >> 10;
    for prc in procfs::process::all_processes().unwrap() {
        // println!("{:?}",prc);
        let mut ruid: u32 = 0;
        let mut virt : u64 = 0;
        let mut res : u64 = 0;
        let mut sha : u64 = 0;

        if let p = prc.unwrap() {

            if let Ok(status ) = p.status() {
                ruid = status.ruid;
            }

            if let Ok(statm) = p.statm() {
                virt = statm.size * page_sizeKB;
                res = statm.resident * page_sizeKB;
                sha = statm.shared * page_sizeKB ;
            }

            println!("{:?}",p.cmdline());

            let username = proc_get_username(ruid);
            if let Ok(stat) = p.stat() {
                // total_time is in seconds
                let data  = proc::ProcInfo {
                    user: username,
                    pid: stat.pid,
                    command: stat.comm,
                    nice : stat.nice,
                    priority : stat.priority,
                    virt : virt,
                    res: res,
                    shr : sha,
                    state : String::from(stat.state),
                    cmdline : p.cmdline().unwrap().iter().map(|x| x.to_string() + " ").collect::<String>()
                };
                map.insert(stat.pid,data);
            }
        }
    }

    let serialized = serde_json::to_string(&map).unwrap();
    serialized
}





fn is_valid_process(pid : i32) -> bool {
    let process = procfs::process::Process::new(pid);
    match process {
        Err(_e) => {
            return false;
        },
        _ => {
            return true;
        }
    }
}

pub fn process_kill(pid : i32) -> std::string::String {
    if is_valid_process(pid) {
        signal::kill(unistd::Pid::from_raw(pid), signal::Signal::SIGTERM).unwrap();
        proc_default_result!(0);
    }
    proc_default_result!(errno::HMIR_ERR_COMM);
}

pub fn process_bind_cpu(pid : i32) -> std::string::String {
    // 进程绑定cpu运行
    if is_valid_process(pid) {
        let core_ids = core_affinity::get_core_ids().unwrap();
        // 目前只能固定绑定第一个cpu
        let core_id = core_ids[0];
        core_affinity::set_for_current(core_id);
        proc_default_result!(0);
    }
    proc_default_result!(errno::HMIR_ERR_COMM);
}


#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("process-all", |_, _| {
        //默认没有error就是成功的
        Ok(process_all())
    })?;

    module.register_method("process-kill", |params, _| {
        //默认没有error就是成功的
        let pid = params.one::<i32>()?;
        Ok(process_kill(pid))
    })?;

    module.register_method("process-bind", |params, _| {
        //默认没有error就是成功的
        let pid = params.one::<i32>()?;
        Ok(process_bind_cpu(pid))
    })?;


    Ok(())
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process_all_it_works() {
        let s = process_all();
        println!("{}",s);
    }


}