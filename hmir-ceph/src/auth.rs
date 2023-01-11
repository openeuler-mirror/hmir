
use crate::command;
use crate::arg;
use crate::ceph_client;
use ceph::cmd;
use ceph::error::RadosError;
use ceph::error::RadosResult;
use log::error;


///list authentication state
pub fn auth_list() -> String {
    command::mon_exec("auth ls")
}

//display requested key
pub fn get_key(client_type: &str, id: &str) -> RadosResult<String> {
    let client = ceph_client::get_ceph_client();
    match client {
        Ok(client) => {
            cmd::auth_get_key(&client, client_type, id)
        },
        Err(e) => {
            error!("{}", format!("{}, Err: {}", arg::CONNECT_FAILED, e));
            Err(RadosError::Error(format!("{}, Err: {}", arg::CONNECT_FAILED, e)))
        }
    }
}