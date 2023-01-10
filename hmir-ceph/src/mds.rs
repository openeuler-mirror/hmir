
use crate::command;


///check running versions of MDSs
///获取mds组件版本信息
pub fn mds_versions() -> String {
    command::mon_exec("mds versions")
}

///获取mds组件状态信息
pub fn mds_stat() -> String {
    command::mon_exec("mds stat")
}

///show mds compatibility settings
pub fn mds_compat_show() -> String {
    command::mon_exec("mds compat show")
}

///查询mds组件元数据信息
pub fn mds_metadata() -> String {
    command::mon_exec("mds metadata")
}