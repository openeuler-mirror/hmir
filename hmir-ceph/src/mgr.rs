
use crate::command;


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

///mgr module ls
///list active mgr modules
pub fn mgr_module_ls() -> String {
    command::mon_exec("mgr module ls")
}

///mgr dump
///dump the latest MgrMap
pub fn mgr_dump() -> String {
    command::mon_exec("mgr dump")
}