use crate::arg;
use crate::ceph_client;
use crate::command;
use log4rs;
use log::{error, info, warn};

///mon组件元数据信息查询
pub fn mon_metadata() -> String {
    command::mon_exec("mon metadata")
}