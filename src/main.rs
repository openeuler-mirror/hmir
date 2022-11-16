
extern crate single_instance;
use single_instance::SingleInstance;
use std::process;
use constants::constants;
use log4rs;
use log::{error,info,warn};
use clap::{Arg,App};

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



fn main() -> anyhow::Result<()> {



    log_init();
    assert_single_instance!();

    let mut app = App::new("hmir");
    let mut matches = app.clone()
        .version(constants::VERSION)
        .author("duanwujie")
        .about("Host management in rust")
        .arg(Arg::with_name("host")
                 .short('h')
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



    Ok(())


}






