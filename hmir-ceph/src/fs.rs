
use crate::command;


///文件系统列表
pub fn fs_list() -> String {
    command::mon_exec("fs ls")
}

///dump all CephFS status
///列出所有分布式文件系统的状态
pub fn fs_dump() -> String {
    command::mon_exec("fs dump")
}