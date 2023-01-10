
use crate::command;


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

///list available mon map features
pub fn mon_feature_ls() -> String {
    command::mon_exec("mon feature ls")
}