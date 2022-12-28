use crate::arg;
use crate::ceph_client;
use crate::command;
use log4rs;
use log::{error, info, warn};

///集群使用率
pub fn df() -> String {
    command::mon_exec("df")
}