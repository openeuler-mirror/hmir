mod svr;
mod ceph;
mod ipmi;
mod net;
mod pkg;
mod proc;
mod kernel;
mod observer;


#[macro_use]
extern crate lazy_static;

extern crate single_instance;
use single_instance::SingleInstance;
use std::process;
use constants::constants;
use log4rs;
use log::{error,info,warn};
use clap::{Arg,App};

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};
use jsonrpsee_core::middleware::{WsMiddleware, Headers, MethodKind, Params};
use std::{time::Instant, net::SocketAddr};

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
    svr::init_services_mg();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    log_init();
    assert_single_instance!();
    init_services();

    let mut app = App::new("hmir");
    let mut matches = app.clone()
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
        app.print_help();
        error!("Argument error with ip {:?}",ip);
        hmir_exit!();
    }

    let port = matches.value_of("port");
    if port == None {
        app.print_help();
        error!("Argument error with ip {:?}",port);
        hmir_exit!();
    }

    // println!("Bind Address : {:?}:{:?}",ip,port);
    let (server_addr, _handle) = run_ws_server(ip.unwrap(),port.unwrap()).await?;
    futures::future::pending().await

}


async fn run_ws_server(ip: &str , port : &str) -> anyhow::Result<(SocketAddr,WsServerHandle)> {

    let url: String =ip.to_owned();
    let url = url + ":" + port;

    let server = WsServerBuilder::default().build(url.parse::<SocketAddr>()?).await?;
    let mut module = RpcModule::new(());

    svr::register_method(& mut module);
    pkg::register_method(& mut module);
    ipmi::register_method(& mut module);
    net::register_method(& mut module);
    proc::register_method(& mut module);
    kernel::register_method(& mut module);
    observer::register_method(& mut module);

    let addr = server.local_addr()?;
    let server_handle = server.start(module)?;
    info!("Sucess start the service with {}:{}",ip,port);


    Ok((addr,server_handle))
}





