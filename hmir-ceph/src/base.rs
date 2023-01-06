use crate::arg;
use crate::ceph_client;
use crate::command;
use log4rs;
use log::{error, info, warn};

///集群使用率
pub fn df() -> String {
    command::mon_exec("df")
}

///集群ID
pub fn fsid() -> String {
    command::mon_exec("fsid")
}

///列出相关服务运行节点
pub fn node_ls() -> String {
    command::mon_exec("node ls")
}

///列出集群状态
pub fn status() -> String {
    command::mon_exec("status")
}

///show mon daemon version
pub fn version() -> String {
    command::mon_exec("version")
}