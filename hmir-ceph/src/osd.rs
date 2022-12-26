use crate::arg;
use crate::ceph_client;
use crate::command;
use log4rs;
use log::{error, info, warn};

///osd树形拓扑
pub fn osd_tree() -> String {
    command::mon_exec("osd tree")
}