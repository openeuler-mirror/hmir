//! 网络管理模块
//! ovs 管理功能：通过unix套接字调用ovsdb Json RPC接口管理ovs网桥，ovsdb Json RPC文档 RFC7047
//!
pub mod ovs;

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};

#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    ovs::register_method(module);
    
    Ok(())
}