//! IPMI管理模块
//!
//!

use jsonrpsee::ws_server::{RpcModule};


#[doc(hidden)]
#[allow(dead_code)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    Ok(())
}