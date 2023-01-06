//! 用于启动终端
//!
//!

use jsonrpsee::ws_server::{RpcModule};
use std::process::{Command,Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use nix::sys::signal;
use nix::unistd;
use hmir_hash::HashWrap;
use std::thread;
use jsonrpsee::http_client::transport::Error;

use log::{error,info};
use tokio::time;

type Pid = Arc<Mutex<u32>>;

use hmir_token::LoginChecker;

lazy_static! {
    static ref tty_id : Pid = {
        Arc::new(Mutex::new(0))
    };
}


macro_rules! ttyd_default_result {
    ($i:expr) =>{
        let mut response = HashWrap::<i32,i32>:: new();
        response.set_code($i);
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}

pub fn ttyd_start() -> String
{
    return futures::executor::block_on(aysnc_ttyd_start());
    // return ttyd_start_test();
}


pub fn org_ttyd_start() -> String
{
    if *tty_id.lock().unwrap() != 0 {
        ttyd_default_result!(0);
    } else {
        tokio::task::spawn(async move {
            //运行在一个不阻塞的线程
            info!("The ttyd has start its execution !");
            if let Ok(mut child) = Command::new("ttyd").arg("-p").arg("5899").arg("bash")
                .stdout(Stdio::null())
                .spawn()
            {
                println!("lock tx.send called");
                *tty_id.lock().unwrap() = child.id();
                println!("before tx.send called");
                println!("tx.send called");
                child.wait().expect("command wasn't running");
                info!("The ttyd has finished its execution!");
            }
        });
        ttyd_default_result!(0);
    }
}

pub async fn aysnc_ttyd_start() -> String
{

    if *tty_id.lock().unwrap() != 0 {
        ttyd_default_result!(0);
    } else {
        let (tx, mut rx) = std::sync::mpsc::channel();
        let thread_join_handle = thread::spawn(move || {
            info!("The ttyd has start its execution !");
            if let Ok(mut child) = Command::new("ttyd").arg("-p").arg("5899").arg("bash")
                .stdout(Stdio::null())
                .spawn()
            {
                *tty_id.lock().unwrap() = child.id();
                let rt = tokio::runtime::Runtime::new().unwrap();
                tx.send("true");
                child.wait().expect("command wasn't running");
                info!("The ttyd has finished its execution!");
            }
        });

        let r = rx.recv_timeout(std::time::Duration::from_millis(500));
        match r {
            Ok(msg) => {
                if msg == "true" {
                    ttyd_default_result!(0);
                }else {
                    ttyd_default_result!(-1);
                }
            }
            _ => {
                ttyd_default_result!(-1);
            }
        }
    }
}



pub fn ttyd_stop() -> String
{
    if ( *tty_id.lock().unwrap() != 0 ){
        let id = *tty_id.lock().unwrap() as i32;
        signal::kill(unistd::Pid::from_raw(id), signal::Signal::SIGHUP).unwrap();
        *tty_id.lock().unwrap() = 0;
    }
    ttyd_default_result!(0);
}



pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("ttyd-start", |_, _| {
        //默认没有error就是成功的

        Ok(ttyd_start())
    })?;

    module.register_method("ttyd-stop", |params, _| {
        //默认没有error就是成功的

        /*
               {
                    "jsonrpc":"2.0",
                    "id":1,
                    "method":"ttyd-stop",
                    "params":["aaaaaaa"]
               }

               {
                    "jsonrpc":"2.0",
                    "id":1,
                    "method":"ttyd-stop"
               }
         */
        let token = params.one::<std::string::String>()?;
        LoginChecker!(token);
        
        Ok(ttyd_stop())


    })?;

    Ok(())

}