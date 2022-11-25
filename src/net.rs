//! 网络管理模块
//! ovs 管理功能：通过unix套接字调用ovsdb Json RPC接口管理ovs网桥，ovsdb Json RPC文档 RFC7047
//!
pub mod ovs;

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};
use std::collections::HashMap;


#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("ovs-check-connection", |_, _| {
        Ok(ovs::check_connection())
    })?;

    module.register_method("ovs-get-ports", |_, _| {
        Ok(ovs::get_ports())
    })?;

    module.register_method("ovs-get-bridges", |_, _| {
        Ok(ovs::get_bridges())
    })?;

    module.register_method("ovs-add-port", |params, _| {
        let port_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs::add_port(port_info))
    })?;

    Ok(())
}