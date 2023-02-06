use ceph::cmd::config_key_exists;
use log::error;
use ceph::error::RadosResult;
use ceph::error::RadosError;
use crate::arg;
use crate::ceph_client;

pub fn exists(key: &str) -> RadosResult<bool> {
    let client = ceph_client::get_ceph_client();
    match client {
        Ok(client) => {
            config_key_exists(&client, key)
        },
        Err(e) => {
            error!("{}", format!("{}, Err: {}", arg::CONNECT_FAILED, e));
            Err(RadosError::Error(format!("{}, Err: {}", arg::CONNECT_FAILED, e)))
        }
    }
}