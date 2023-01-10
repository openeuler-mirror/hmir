
use crate::command;

///pg列表
pub fn pg_list() -> String {
    command::mon_exec("pg ls")
}