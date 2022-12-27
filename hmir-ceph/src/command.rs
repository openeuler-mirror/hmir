use crate::arg;
use crate::ceph_client;
use log4rs;
use log::{error, info, warn};

///ceph_mon_command命令执行接口封装
pub fn mon_exec(cmd: &str) -> String {
    let client = ceph_client::get_ceph_client();
    match client {
        Ok(client) => {
            match client.ceph_mon_command("prefix", cmd, Some("json")) {
                Ok((outbuf, outs)) => {
                    match outbuf {
                        Some(outbuf) => { return outbuf; },
                        None => { return String::new(); }
                    }
                    match outs {
                        Some(outs) => { return outs; },
                        None => { return String::new(); }
                    }
                },
                Err(e) => {
                    format!("Exec {} failed: {:?}", cmd, e)
                }
            }
        },
        Err(e) => {
            format!("{}", arg::CONNECT_FAILED)
        }
    }
}