//! ovs网络能力
//! ovs-query：网桥信息查询接口，通过unix套接字调用ovsdb Json RPC接口管理ovs网桥，ovsdb Json RPC文档 RFC7047
//! ovs-vsctl：网桥设置，调用 ovs-vsctl 命令执行
//！ovs-ofctl：流控能力，调用 ovs-ofctl 命令执行

use jsonrpsee::ws_server::RpcModule;
use std::collections::HashMap;
use hmir_ovs::*;

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    ovs_query::register_method(module);
    ovs_vsctl::register_method(module);
    ovs_ofctl::register_method(module);

    Ok(())
}