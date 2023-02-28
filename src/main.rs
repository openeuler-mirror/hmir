mod net;
mod pkg;
mod ttyd;
mod ssh;
mod pam;
mod user;
// mod token;

#[cfg(target_os = "linux")]
mod ceph;

#[cfg(target_os = "linux")]
mod proc;
#[cfg(target_os = "linux")]
mod ipmi;
#[cfg(target_os = "linux")]
mod svr;
#[cfg(target_os = "linux")]
mod kernel;
#[cfg(target_os = "linux")]
mod observer;
mod tokenmgr;

#[cfg(target_os = "linux")]
mod virt;

mod sys;
// mod tensorflow;

#[macro_use]
extern crate lazy_static;

extern crate single_instance;

use single_instance::SingleInstance;
use std::{io, process};
use constants::constants;
use log4rs;
use log::{error, info};
use clap::{Arg, App};

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder, WsServerHandle};
use jsonrpsee::core::middleware::{self, Headers, MethodKind, Params};
use std::time::Instant;

use std::{net::SocketAddr, thread, time};
use std::fs::File;
use nix::{libc};
use std::fs::OpenOptions;
use std::os::fd::AsRawFd;
use nix::errno::{Errno, errno};
use nix::libc::{F_GETFD, F_SETFD, FD_CLOEXEC};

// use hmir_dpkg;

#[derive(Clone)]
struct Timings;

impl middleware::WsMiddleware for Timings {
    type Instant = Instant;

    fn on_connect(&self, remote_addr: SocketAddr, headers: &Headers) {
        println!("[Middleware::on_connect] remote_addr {}, headers: {:?}", remote_addr, headers);
    }

    fn on_request(&self) -> Self::Instant {
        Instant::now()
    }

    fn on_call(&self, name: &str, params: Params, kind: MethodKind) {
        println!("[Middleware::on_call] method: '{}', params: {:?}, kind: {}", name, params, kind);
    }

    fn on_result(&self, name: &str, succeess: bool, started_at: Self::Instant) {
        println!("[Middleware::on_result] '{}', worked? {}, time elapsed {:?}", name, succeess, started_at.elapsed());
    }

    fn on_response(&self, result: &str, started_at: Self::Instant) {
        println!("[Middleware::on_response] result: {}, time elapsed {:?}", result, started_at.elapsed());
    }

    fn on_disconnect(&self, remote_addr: SocketAddr) {
        println!("[Middleware::on_disconnect] remote_addr: {}", remote_addr);
    }
}

macro_rules! assert_single_instance {
    ()=>{
        let __instance = SingleInstance::new("the-hmir-process-lock").unwrap();
        if(!__instance.is_single() ){
            println!("Already have an instance running");
            error!("Already have an instance running");
            process::exit(1);
        }
    }
}

macro_rules! hmir_exit {
    () => {
        process::exit(1);
    };
}

fn log_init()
{
    let log = log4rs::init_file("/etc/hmir/log4rs.yaml", Default::default());
    match log {
        Err(e) => {
            println!("Err for init log : {}", e);
            process::exit(1);
        }
        _ => ()
    }
}

#[doc(hidden)]
fn init_services() {
    #[cfg(target_os = "linux")]
    {
        svr::init_services_mg();
        proc::init_proc_mg();
        sys::init_sysinfo();
    }
}




fn singleton_process()
{

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("/var/run/hmir.lock");
    if let Ok(file) = file {
        let fd = file.as_raw_fd();

        unsafe {
            let mut flags = libc::fcntl(fd,F_GETFD);
            flags = flags | FD_CLOEXEC;
            libc::fcntl(fd,F_SETFD,flags);
        }

        let res = unsafe {
            let res = libc::flock(fd, libc::LOCK_EX);
            if res == 0 {
                println!("lock file success");
            }
        };
        let e = errno();
        println!("E is {}",e);

        // if let Ok(_) = res {
        // } else {
        //     println!("Already have an instance running");
        //     error!("Already have an instance running");
        //     process::exit(1);
        // }
    }else {
        println!("Open the /var/run/hmir.lock failed");
    }
}


fn daemonize() {
    // 将当前进程设置为守护进程
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            panic!("Failed to fork");
        } else if pid > 0 {
            // 父进程退出
            std::process::exit(0);
        }
        // 创建新会话
        libc::setsid();

        // 将当前工作目录更改为根目录
        let c_str = std::ffi::CString::new("/").unwrap();
        let root_dir = c_str.as_ptr();
        if libc::chdir(root_dir) < 0 {
            panic!("Failed to change working directory");
        }
        // 关闭所有文件描述符
        for fd in 0..libc::sysconf(libc::_SC_OPEN_MAX) {
            libc::close(fd as libc::c_int);
        }
        // 重定向标准输入、输出和错误
        let c_str = std::ffi::CString::new("/dev/null").unwrap();
        let dev_null = c_str.as_ptr();
        let fd = libc::open(dev_null, libc::O_RDWR);
        if fd < 0 {
            panic!("Failed to open /dev/null");
        }
        libc::dup2(fd, libc::STDIN_FILENO);
        libc::dup2(fd, libc::STDOUT_FILENO);
        libc::dup2(fd, libc::STDERR_FILENO);
        if fd > 2 {
            libc::close(fd);
        }
    }
}


fn main() -> anyhow::Result<()> {

    assert_single_instance!();
    // daemonize();
    return run();
}

#[tokio::main]
async fn run() -> anyhow::Result<()> {
    log_init();
    // singleton_process();
    init_services();

    let mut app = App::new("hmir");
    let matches = app.clone()
        .version(constants::VERSION)
        .author("duanwujie")
        .about("Host management in rust")
        .arg(Arg::with_name("host")
            .short('x')
            .long("host")
            .takes_value(true)
            .help("The host ip address"))
        .arg(Arg::with_name("port")
            .short('p')
            .long("port")
            .takes_value(true)
            .help("The port"))
        .get_matches();

    let ip = matches.value_of("host");
    if ip == None {
        app.print_help()?;
        error!("Argument error with ip {:?}",ip);
        hmir_exit!();
    }

    let port = matches.value_of("port");
    if port == None {
        app.print_help()?;
        error!("Argument error with ip {:?}",port);
        hmir_exit!();
    }

    // println!("Bind Address : {:?}:{:?}",ip,port);
    let (_server_addr, _handle) = run_ws_server(ip.unwrap(), port.unwrap()).await?;
    futures::future::pending().await
}

// use jsonrpsee_core::client::Client;


async fn run_ws_server(ip: &str, port: &str) -> anyhow::Result<(SocketAddr, WsServerHandle)> {
    let url: String = ip.to_owned();
    let url = url + ":" + port;

    let server = WsServerBuilder::new().set_middleware(Timings).build(url.parse::<SocketAddr>()?).await?;
    // let server = WsServerBuilder::default().build(url.parse::<SocketAddr>()?).await?;
    let mut module = RpcModule::new(());


    // println!("{:?}",client);

    #[cfg(target_os = "linux")]
    {
        svr::register_method(&mut module)?;
        ipmi::register_method(&mut module)?;
        proc::register_method(&mut module)?;
        kernel::register_method(&mut module)?;
        observer::register_method(&mut module)?;
        ceph::ceph::register_method(&mut module)?;
        virt::register_method(&mut module)?;
        sys::register_method(&mut module)?;
    }


    pkg::register_method(&mut module)?;
    net::register_method(&mut module)?;
    ttyd::register_method(&mut module)?;
    pam::register_method(&mut module)?;
    ssh::register_method(&mut module)?;

    let addr = server.local_addr()?;
    let server_handle = server.start(module)?;
    info!("Success start the service with {}:{}",ip,port);
    Ok((addr, server_handle))
}




