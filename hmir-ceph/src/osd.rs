use crate::arg;
use crate::ceph_client;
use log4rs;
use log::{error, info, warn};

///osd树形拓扑
pub fn osd_tree() -> String {
    let client = ceph_client::get_ceph_client();
    match client {
        Ok(client) => {
            match client.ceph_mon_command("prefix", "osd tree", Some("json")) {
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
                    format!("Get osd tree failed: {:?}", e)
                }
            }
        },
        Err(e) => {
            format!("{}", arg::CONNECT_FAILED)
        }
    }
}