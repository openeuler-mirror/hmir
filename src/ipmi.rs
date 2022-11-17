//! IPMI管理模块
//!
//!

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};


#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    Ok(())
}