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
use crate::command;
use hmir_hash::hmir_result::HmirResult;

#[cfg(unix)]
///获取ceph集群的基本状态信息
pub fn ceph_status() -> String {
    command::mon_exec("status")
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

#[macro_export]
macro_rules! ceph_client {
    ( $x:expr ) => {
        $x = ceph_client::get_ceph_client();
        match $x {
            Ok(_) => {},
            Err(e) => return HmirResult::new(1,
                            format!("Failed to connect ceph, {}", e.to_string()),
                            String::from(""))
        }
    }
}