
extern crate single_instance;
use single_instance::SingleInstance;
use std::process;
use constants::constants;
use log4rs;
use log::{error,info,warn};


macro_rules! assert_single_instance{
    ()=>{
        let __instance = SingleInstance::new(constants::LOCKFILE).unwrap();
        if(!__instance.is_single() ){
            error!("Already have an instance running");
            process::exit(1);
        }
    }
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


    todo!()


}






