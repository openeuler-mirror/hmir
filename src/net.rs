//! 网络管理模块
//!
pub mod ovs;
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};

#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    ovs::register_method(module);
    
    Ok(())
}