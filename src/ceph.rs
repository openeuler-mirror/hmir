//! CEPH管理模块
//!
//!

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};
use hmir_ceph::ceph_client;
use hmir_hash::HashWrap;


#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {
    //The svr module

    module.register_method("ceph-cluster-stat", |_, _| {
        //获取ceph集群状态
        Ok(ceph_cluster_stat())
    })?;

    Ok(())
}

pub fn ceph_cluster_stat() -> String {
    ceph_client::ceph_status()
}