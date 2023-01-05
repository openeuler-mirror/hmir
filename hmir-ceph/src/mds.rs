use crate::arg;
use crate::ceph_client;
use crate::command;
use log4rs;
use log::{error, info, warn};

///check running versions of MDSs
///获取mds组件版本信息
pub fn mds_versions() -> String {
    command::mon_exec("mds versions")
}