use crate::arg;
use crate::ceph_client;
use crate::command;
use log4rs;
use log::{error, info, warn};

///mgr metadata
pub fn mgr_metadata() -> String {
    command::mon_exec("mgr metadata")
}

///mgr versions
pub fn mgr_versions() -> String {
    command::mon_exec("mgr versions")
}

///mgr services
///list service endpoints provided by mgr modules
pub fn mgr_services() -> String {
    command::mon_exec("mgr services")
}