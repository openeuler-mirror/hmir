use crate::arg;
use crate::ceph_client;
use crate::command;
use log4rs;
use log::{error, info, warn};

///list authentication state
pub fn auth_list() -> String {
    command::mon_exec("auth ls")
}