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
use hmir_hash::HashWrap;
use hmir_errno::errno;
use nix::sys::signal;
use nix::unistd;
use nix::libc;
use std::ffi::CStr;
use hmir_protocol::proc;
use sysinfo::{System, SystemExt};
use std::{thread, time};
use std::sync::{Mutex};


extern crate core_affinity;

macro_rules! proc_default_result {
    ($i:expr) =>{
        let mut response = HashWrap::<i32,proc::ProcInfo>:: new();
        response.set_code($i);
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}

lazy_static! {
    static ref SYS_PROCESS_CACHE: Mutex<String> = Mutex::new(String::new());
}


fn proc_get_username(uid: libc::uid_t) -> String
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

pub fn get_cpu_total_occupy() -> f64
{
    let stat = procfs::KernelStats::new().unwrap();
    return (stat.total.user + stat.total.nice + stat.total.system + stat.total.idle) as f64;
}

pub fn get_cpu_proc_occupy(pid: i32) -> f64
{
    let prc = procfs::process::Process::new(pid);
    match prc {
        Ok(p) => {
            if let Ok(stat) = p.stat() {
                return (stat.utime + stat.stime + stat.cutime as u64 + stat.cstime as u64) as f64;
            }
        },
        _ => {
            return 0.0;
        }
    }

    return 0.0;
}





pub fn sys_total_memory() -> f64 {
    let sys = System::new_all();
    sys.total_memory() as f64
}

fn update_all_cpu_usage(map: &mut HashWrap::<i32, proc::ProcInfo>)
{
    for (k,v)  in map.result.iter_mut() {
        v.cpu_usage = get_cpu_proc_occupy(*k);
    }
    let total_cpu_time1 = get_cpu_total_occupy();
    std::thread::sleep(std::time::Duration::from_millis(200));
    let total_cpu_time2 = get_cpu_total_occupy();

    for (k,v)  in map.result.iter_mut() {
        v.cpu_usage = get_cpu_proc_occupy(*k) - v.cpu_usage;
    }
    if (total_cpu_time2 - total_cpu_time1).abs() > f64::EPSILON
    {
        for (_k,v)  in map.result.iter_mut() {
            v.cpu_usage = (10000.0 * v.cpu_usage / (total_cpu_time2 - total_cpu_time1)).round()/100.0;
        }
    }
}

fn update_process_all()
{
    *SYS_PROCESS_CACHE.lock().unwrap() = _process_all();
}

#[doc(hidden)]
pub fn init_proc_mg()  {
    tokio::task::spawn(async{
        //运行在一个不阻塞的线程
        let sec = time::Duration::from_millis(1000);
        loop {
            update_process_all();
            thread::sleep(sec);
        }
    });
}

pub fn _process_all() -> std::string::String
{
    let mut map = HashWrap::<i32, proc::ProcInfo>::new();
    let page_size_kb = proc_get_pagesize() as u64 >> 10;

    let total_memory = sys_total_memory();

    for prc in procfs::process::all_processes().unwrap() {
        // println!("{:?}",prc);
        let mut ruid: u32 = 0;
        let mut virt: u64 = 0;
        let mut res: u64 = 0;
        let mut sha: u64 = 0;

        if let Ok(p) = prc {
            if let Ok(status) = p.status() {
                ruid = status.ruid;
            }

            if let Ok(statm) = p.statm() {
                virt = statm.size * page_size_kb;
                res = statm.resident * page_size_kb;
                sha = statm.shared * page_size_kb;
            }

            let username = proc_get_username(ruid);
            if let Ok(stat) = p.stat() {
                // total_time is in seconds
                let data = proc::ProcInfo {
                    user: username,
                    pid: stat.pid,
                    command: stat.comm,
                    nice: stat.nice,
                    priority: stat.priority,
                    virt: virt,
                    res: res,
                    shr: sha,
                    state: String::from(stat.state),
                    cpu_usage: 0.0,
                    mem_usage: ((res as f64 * 1024.0 / total_memory) * 10_000.0).round() / 100.0,
                    cmdline: p.cmdline().unwrap().iter().map(|x| x.to_string() + " ").collect::<String>(),
                };

                map.insert(stat.pid, data);
            }
        }
    }

    update_all_cpu_usage(&mut map);
    let serialized = serde_json::to_string(&map).unwrap();
    serialized
}

pub fn process_all() -> std::string::String
{
    let result = (*SYS_PROCESS_CACHE.lock().unwrap()).clone();
    result
}


fn is_valid_process(pid: i32) -> bool {
    let process = procfs::process::Process::new(pid);
    match process {
        Err(_e) => {
            return false;
        }
        _ => {
            return true;
        }
    }
}

pub fn process_kill(pid: i32) -> std::string::String {
    if is_valid_process(pid) {
        signal::kill(unistd::Pid::from_raw(pid), signal::Signal::SIGTERM).unwrap();
        proc_default_result!(0);
    }
    proc_default_result!(errno::HMIR_ERR_COMM);
}

pub fn process_bind_cpu(pid: i32) -> std::string::String {
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
pub fn register_method(module: &mut RpcModule<()>) -> anyhow::Result<()> {
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
        println!("{}", s);
    }

    #[test]
    fn process_cpu_test() {
        let data = proc::ProcInfo {
            pid: 1,
            user: "".to_string(),
            priority: 0,
            nice: 0,
            virt: 0,
            res: 0,
            shr: 0,
            state: "".to_string(),
            cpu_usage: 0.0,
            mem_usage: 0.0,
            command: "".to_string(),
            cmdline: "".to_string(),
        };
        let mut map = HashWrap::<i32, proc::ProcInfo>::new();
        map.insert(data.pid, data);
        update_all_cpu_usage(& mut map);

        for (k,v) in map.result.into_iter() {
            println!("----- {} : {}",k,v.cpu_usage);
        }

    }
}