use crate::arg;
use crate::ceph_client;
use crate::command;
use log4rs;
use log::{error, info, warn};

///mon组件元数据信息查询
pub fn mon_metadata() -> String {
    command::mon_exec("mon metadata")
}

///mon组件状态获取
pub fn mon_status() -> String {
    command::mon_exec("mon_status")
}

///dump formatted monmap
pub fn mon_dump() -> String {
    command::mon_exec("mon dump")
}

///mon组件版本信息查询
pub fn mon_versions() -> String {
    command::mon_exec("mon versions")
}