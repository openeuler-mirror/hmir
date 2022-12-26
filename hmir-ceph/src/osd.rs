use crate::arg;
use crate::ceph_client;
use crate::command;
use log4rs;
use log::{error, info, warn};

///osd树形拓扑
pub fn osd_tree() -> String {
    command::mon_exec("osd tree")
}

///osd组件版本查询
pub fn osd_versions() -> String {
    command::mon_exec("osd versions")
}

//osd组件元数据信息查询
pub fn osd_metadata() -> String {
    command::mon_exec("osd metadata")
}