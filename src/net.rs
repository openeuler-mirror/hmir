//! 网络管理模块
//! 已增加功能，如下：
//! 1、ovs网桥管理
//! 2、linux legacy网桥管理
//! 

pub mod ovs;
pub mod linuxbr;
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};

#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    ovs::register_method(module);

    linuxbr:: register_method(module);

    Ok(())
}