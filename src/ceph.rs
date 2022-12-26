//! CEPH管理模块
//!
//!

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};
use hmir_ceph::ceph_client;
use hmir_ceph::pool;
use hmir_hash::HashWrap;


#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {
    //The svr module

    module.register_method("ceph-cluster-stat", |_, _| {
        //获取ceph集群状态
        Ok(ceph_cluster_stat())
    })?;
    
    ///pool
    module.register_method("ceph-pool-list", |_, _| {
        //获取ceph集群存储池列表
        Ok(ceph_pool_list())
    })?;
    
    module.register_method("ceph-pool-stats", |_, _| {
        //获取ceph集群存储池状态
        Ok(ceph_pool_stats())
    })?;
    
    Ok(())
}

pub fn ceph_cluster_stat() -> String {
    ceph_client::ceph_status()
}

///pool

pub fn ceph_pool_list() -> String {
    pool::pool_list()
}

pub fn ceph_pool_stats() -> String {
    pool::pool_stats()
}