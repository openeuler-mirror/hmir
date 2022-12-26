use crate::arg;
use crate::ceph_client;
use log4rs;
use log::{error, info, warn};

///存储池列表
pub fn pool_list() -> String {
    let client = ceph_client::get_ceph_client();
    match client {
        Ok(client) => {
            match client.ceph_mon_command("prefix", "osd pool ls", Some("json")) { 
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
                    format!("Get pool list failed: {:?}", e)
                }
            }
        },
        Err(e) => {
            format!("{}", arg::CONNECT_FAILED)
        }
    }
}

///存储池状态
pub fn pool_stats() -> String {
    let client = ceph_client::get_ceph_client();
    match client {
        Ok(client) => {
            match client.ceph_mon_command("prefix", 
                                          "osd pool stats", 
                                          Some("json")) {
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
                    format!("Get pool stats failed: {:?}", e)
                }
            }
        },
        Err(e) => {
            format!("{}", arg::CONNECT_FAILED)
        }
    }
}