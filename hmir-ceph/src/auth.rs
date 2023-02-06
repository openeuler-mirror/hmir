// extern crate serde_json;

use std::fmt::Debug;
use crate::command;
use crate::arg;
use crate::ceph_client;
use ceph::cmd::*;
use ceph::error::RadosError;
use ceph::error::RadosResult;
use log::error;
use serde_json::json;
use serde::*;

///list authentication state
pub fn auth_list() -> String {
    command::mon_exec("auth ls")
}

//display requested key
pub fn get_key(client_type: &str, id: &str) -> RadosResult<String> {
    let client = ceph_client::get_ceph_client();
    match client {
        Ok(client) => {
            auth_get_key(&client, client_type, id)
        },
        Err(e) => {
            error!("{}", format!("{}, Err: {}", arg::CONNECT_FAILED, e));
            Err(RadosError::Error(format!("{}, Err: {}", arg::CONNECT_FAILED, e)))
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Caps {
    pub mds: String,
    pub mgr: String,
    pub mon: String,
    pub osd: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthResult {
    pub entity: String,
    pub key: String,
    pub caps: Caps,
}

#[test]
pub fn test() {
    let s = auth_get("client", "admin");
    match s {
        Ok(s) => {
            let s = json!(&s).to_string();
            println!("Ok {:?}",s)
        },
        Err(s) => {
            println!("error {:?}", format!("{:?}",s));
        },
    }
}

//write keyring file with requested key
pub fn auth_get(client_type: &str, id: &str) -> RadosResult<Vec<AuthResult>> {
    let client = ceph_client::get_ceph_client()?;
    let cmd = json!({
        "prefix": "auth get",
        "entity": format!("{}.{}", client_type, id),
        "format": "json"
    });
    let result = client.ceph_mon_command_without_data(&cmd)?;
    let return_data = String::from_utf8(result.0)?;
    println!("return_data : {}", return_data);
    Ok(serde_json::from_str(&return_data)?)
}

//add auth info for mgr
pub fn add_mgr(id: &str) -> RadosResult<()> {
    let client = ceph_client::get_ceph_client()?;
    mgr_auth_add(&client, id, false)
}

//add auth info for osd
pub fn add_osd(id: u64) -> RadosResult<()> {
    let client = ceph_client::get_ceph_client()?;
    osd_auth_add(&client, id, false)
}