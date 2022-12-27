use crate::arg;
use crate::ceph_client;
use crate::command;
use log4rs;
use log::{error, info, warn};

///pg列表
pub fn pg_list() -> String {
    command::mon_exec("pg ls")
}