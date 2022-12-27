//! CEPH管理模块
//!
//!

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};
use hmir_ceph::ceph_client;
use hmir_ceph::pool;
use hmir_ceph::osd;
use hmir_ceph::mon;
use hmir_hash::HashWrap;


#[doc(hidden)]
pub fn register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    //The svr module

    ///mon
    ceph_mon_register_method(module);
    
    
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
    
    ///osd
    module.register_method("ceph-osd-tree", |_, _| {
        //获取osd拓扑结构
        Ok(ceph_osd_tree())
    })?;

    module.register_method("ceph-osd-versions", |_, _| {
        //获取osd组件版本信息
        Ok(ceph_osd_versions())
    })?;

    module.register_method("ceph-osd-metadata", |_, _| {
        //获取osd组件元数据信息
        Ok(ceph_osd_metadata())
    })?;

    module.register_method("ceph-osd-perf", |_, _| {
        //osd性能测试
        Ok(ceph_osd_perf())
    })?;
    
    ///osd crush
    module.register_method("ceph-osd-crush-rule-dump", |_, _| {
        //获取osd的crush规则
        Ok(ceph_osd_crush_rule_dump())
    })?;
    
    Ok(())
}


pub fn ceph_mon_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    ///mon metadata
    module.register_method("ceph-mon-metadata", |_, _| {
        //获取mon的元数据信息
        Ok(mon::mon_metadata())
    })?;

    ///mon status
    module.register_method("ceph-mon-status", |_, _| {
        //获取mon集群的状态
        Ok(mon::mon_status())
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


///osd

pub fn ceph_osd_tree() -> String {
    osd::osd_tree()
}

pub fn ceph_osd_versions() -> String {
    osd::osd_versions()
}

pub fn ceph_osd_metadata() -> String {
    osd::osd_metadata()
}

pub fn ceph_osd_perf() -> String {
    osd::osd_perf()
}

///osd crush
pub fn ceph_osd_crush_rule_dump() -> String {
    osd::osd_crush_rule_dump()
}