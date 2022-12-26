use crate::arg;
use crate::ceph_client;
use crate::command;
use log4rs;
use log::{error, info, warn};

///存储池列表
pub fn pool_list() -> String {
    command::mon_exec("osd pool ls")
}

///存储池状态
pub fn pool_stats() -> String {
    command::mon_exec("osd pool stats")
}