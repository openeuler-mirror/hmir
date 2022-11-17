//! 软件包管理模块
//!
//! 支持的系统
//! - debian系
//! - euler系
//! 支持以下的请求
//! -
//!

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};


#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    Ok(())
}