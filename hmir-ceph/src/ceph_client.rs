#![allow(unused_imports)]
extern crate ceph_rust;
extern crate ceph;
extern crate libc;


use ceph::ceph as ceph_helpers;
use log4rs;
use log::{error, info, warn};
use crate::arg;

#[cfg(unix)]
pub fn ceph_status() -> String {
    let conn_result = ceph_helpers::connect_to_ceph( arg::ADMIN, arg::CONF_PATH);
    if conn_result.is_err() {
        let err_msg = format!("Failed to connect ceph, err: {:?}", conn_result.err());
        error!("{}", err_msg);
        err_msg
    } else {
        let cluster = conn_result.unwrap();
        format!("{:?}", cluster.rados_stat_cluster().unwrap())
    }
}