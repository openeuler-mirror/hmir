//! CEPH管理模块
//!
//!

use jsonrpsee::ws_server::{RpcModule, WsServerBuilder,WsServerHandle};
use hmir_ceph::ceph_client;
use hmir_ceph::pool;
use hmir_ceph::osd;
use hmir_ceph::mon;
use hmir_ceph::pg;
use hmir_ceph::base;
use hmir_ceph::auth;
use hmir_ceph::fs;
use hmir_ceph::mgr;
use hmir_ceph::mds;
use hmir_hash::HashWrap;


#[doc(hidden)]
pub fn register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    //The svr module

    ///mon
    ceph_mon_register_method(module);
    
    ///pg
    ceph_pg_register_method(module);
    
    ///base
    ceph_base_register_method(module);
    
    ///auth
    ceph_auth_register_method(module);
    
    ///fs
    ceph_fs_register_method(module);
    
    ///mgr
    ceph_mgr_register_method(module);
    
    ///mds
    ceph_mds_register_method(module);
    
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

///mon method register
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

    ///mon dump
    module.register_method("ceph-mon-dump", |_, _| {
        //输出格式化的mon map
        Ok(mon::mon_dump())
    })?;

    ///mon versions
    module.register_method("ceph-mon-versions", |_, _| {
        Ok(mon::mon_versions())
    })?;
    
    ///列出可用的mon map特性
    module.register_method("ceph-mon-feature-ls", |_, _| {
        Ok(mon::mon_feature_ls())
    })?;
    
    Ok(())
}


///pg method register
pub fn ceph_pg_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    ///pg list
    module.register_method("ceph-pg-list", |_, _| {
        //获取mon的元数据信息
        Ok(pg::pg_list())
    })?;

    Ok(())
}

///base method register
pub fn ceph_base_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    ///集群基础命令
    module.register_method("ceph-df", |_, _| {
        //集群使用率
        Ok(base::df())
    })?;

    module.register_method("ceph-fsid", |_, _| {
        //集群ID
        Ok(base::fsid())
    })?;
    
    module.register_method("ceph-node-ls", |_, _| {
        //集群ID
        Ok(base::node_ls())
    })?;
    
    Ok(())
}

///auth method register
pub fn ceph_auth_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    ///集群认证相关
    module.register_method("ceph-auth-list", |_, _| {
        //list authentication state
        Ok(auth::auth_list())
    })?;

    Ok(())
}

///fs method register
pub fn ceph_fs_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    ///集群文件系统相关
    module.register_method("ceph-fs-list", |_, _| {
        //list fs
        Ok(fs::fs_list())
    })?;

    module.register_method("ceph-fs-dump", |_, _| {
        //list fs status
        Ok(fs::fs_dump())
    })?;

    Ok(())
}

///mgr method register
pub fn ceph_mgr_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    ///mgr metadata
    module.register_method("ceph-mgr-metadata", |_, _| {
        //mgr metadata
        Ok(mgr::mgr_metadata())
    })?;

    ///mgr versions
    module.register_method("ceph-mgr-versions", |_, _| {
        //mgr versions
        Ok(mgr::mgr_versions())
    })?;

    ///mgr services
    module.register_method("ceph-mgr-services", |_, _| {
        //列出MGR模块提供的服务端点
        Ok(mgr::mgr_services())
    })?;

    ///mgr module ls
    module.register_method("ceph-mgr-module-ls", |_, _| {
        //列出活跃的MGR模块
        Ok(mgr::mgr_module_ls())
    })?;

    ///mgr dump
    module.register_method("ceph-mgr-dump", |_, _| {
        //列出最近的mgr map状态
        Ok(mgr::mgr_dump())
    })?;
    
    Ok(())
}

///mds method register
pub fn ceph_mds_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    ///元数据服务器组件相关
    module.register_method("ceph-mds-versions", |_, _| {
        //获取mds组件版本信息
        Ok(mds::mds_versions())
    })?;

    module.register_method("ceph-mds-stat", |_, _| {
        //获取mds组件状态信息
        Ok(mds::mds_stat())
    })?;

    module.register_method("ceph-mds-compat-show", |_, _| {
        //显示MDS兼容性设置
        Ok(mds::mds_compat_show())
    })?;

    module.register_method("ceph-mds-metadata", |_, _| {
        //查询mds组件元数据信息
        Ok(mds::mds_metadata())
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