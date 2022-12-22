#![allow(unused_imports)]
extern crate ceph_rust;
extern crate ceph;
extern crate libc;


use std::ptr::null;
use ceph::ceph as ceph_helpers;
use ceph::ceph::Rados;
use ceph::error::RadosError;
use libc::stat;
use log4rs;
use log::{error, info, warn};
use crate::arg;

#[cfg(unix)]
///获取ceph集群的基本状态信息
pub fn ceph_status() -> String {
    let client = get_ceph_client();
    match client {
        Ok(client) => {
            match client.ceph_mon_command("prefix", "status", Some("json")) {
                Ok((outbuf, outs)) => {
                    match outbuf { 
                        Some(outbuf) => { return outbuf; },
                        None => { return String::new(); }
                    }
                    match outs { 
                        Some(outs) => { return outs; },
                        None => { return String::new();
                        }
                    }
                },
                Err(e) => {
                    format!("Get status failed: {:?}", e)
                }
            }
        },
        Err(e) => {
            format!("{}", arg::CONNECT_FAILED)
        }
    }
}

///获取ceph集群的client链接
pub fn get_ceph_client() -> Result<Rados, RadosError> {
    let conn_result = ceph_helpers::connect_to_ceph( arg::ADMIN, arg::CONF_PATH);
    match conn_result { 
        Ok(conn) => Ok(conn),
        Err(e) => {
            error!("{}", format!("Failed to connect ceph, err: {:?}", e));
            Err(e)
        }
    }
}