//! CEPH管理模块
//!
//!

use jsonrpsee::ws_server::{RpcModule};


#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {
    //The svr module
    Ok(())
}