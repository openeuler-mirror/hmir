
use crate::command;


///list authentication state
pub fn auth_list() -> String {
    command::mon_exec("auth ls")
}