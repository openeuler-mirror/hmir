mod net;
mod pkg;
mod ttyd;
mod wsclient;
mod ssh;
mod pam;
mod clientmgr;
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

#[macro_use]
extern crate lazy_static;

extern crate single_instance;
use single_instance::SingleInstance;
use std::process;
use constants::constants;
use log4rs;
use log::{error,info};
use clap::{Arg,App};

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};
use jsonrpsee::core::middleware::{self, Headers, MethodKind, Params};
use std::time::Instant;

use std::{net::SocketAddr};


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

macro_rules! assert_single_instance{
    ()=>{
        let __instance = SingleInstance::new(constants::LOCKFILE).unwrap();
        if(!__instance.is_single() ){
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

fn log_init ()
{
    let log = log4rs::init_file("/etc/hmir/log4rs.yaml",Default::default());
    match log {
        Err(e) => {
            println!("Err for init log : {}",e);
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
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    #[cfg(target_os = "linux")]
    {
        log_init();
        assert_single_instance!();
    }



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
    let (_server_addr, _handle) = run_ws_server(ip.unwrap(),port.unwrap()).await?;
    futures::future::pending().await

}

// use jsonrpsee_core::client::Client;


async fn run_ws_server(ip: &str , port : &str) -> anyhow::Result<(SocketAddr,WsServerHandle)> {

    let url: String =ip.to_owned();
    let url = url + ":" + port;

    let server = WsServerBuilder::new().set_middleware(Timings).build(url.parse::<SocketAddr>()? ).await?;
    // let server = WsServerBuilder::default().build(url.parse::<SocketAddr>()?).await?;
    let mut module = RpcModule::new(());


    // println!("{:?}",client);

    #[cfg(target_os  = "linux")]
    {
        svr::register_method(& mut module)?;
        ipmi::register_method(& mut module)?;
        proc::register_method(& mut module)?;
        kernel::register_method(& mut module)?;
        observer::register_method(& mut module)?;
        ceph::register_method(& mut module)?;
        virt::register_method(& mut module)?;
    }


    pkg::register_method(& mut module)?;
    net::register_method(& mut module)?;
    ttyd::register_method(& mut module)?;
    pam::register_method(& mut module)?;
    ssh::register_method(& mut module)?;

    let addr = server.local_addr()?;
    let server_handle = server.start(module)?;
    info!("Sucess start the service with {}:{}",ip,port);
    Ok((addr,server_handle))
}




