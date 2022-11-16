
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};
use std::sync::RwLock;
use hmir_hash::HashWrap;

use hmir_systemd::{
    build_blocking_client,
    manager::blocking::{OrgFreedesktopSystemd1Manager},
    models::{Unit,IntoModel},
    SystemdObjectType,
};


pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {


    Ok(())
}