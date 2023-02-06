//! CEPH管理模块
//!
//!

use jsonrpsee::ws_server::{RpcModule};
use hmir_ceph::ceph_client;
use hmir_ceph::pool;
use hmir_ceph::mon;
use hmir_ceph::pg;
use hmir_ceph::base;
use hmir_ceph::auth;
use hmir_ceph::fs;
use hmir_ceph::mgr;
use hmir_ceph::mds;
use hmir_ceph::config_key;
use std::collections::HashMap;
use serde_json::json;
use log::{error,info};

use crate::ceph::osd::ceph_osd_register_method;
#[doc(hidden)]
pub fn register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    //The svr module

    ceph_mon_register_method(module)?;
    
    ceph_pg_register_method(module)?;
    
    ceph_base_register_method(module)?;
    
    ceph_auth_register_method(module)?;
    
    ceph_fs_register_method(module)?;
    
    ceph_mgr_register_method(module)?;
    
    ceph_mds_register_method(module)?;

    ceph_osd_register_method(module)?;
    
    ceph_osd_pool_register_method(module)?;
    
    ceph_config_key_register_method(module);
    
    module.register_method("ceph-cluster-stat", |_, _| {
        //获取ceph集群状态
        Ok(ceph_cluster_stat())
    })?;
    
    Ok(())
}

///mon method register
pub fn ceph_mon_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    module.register_method("ceph-mon-metadata", |_, _| {
        //获取mon的元数据信息
        Ok(mon::mon_metadata())
    })?;

    module.register_method("ceph-mon-status", |_, _| {
        //获取mon集群的状态
        Ok(mon::mon_status())
    })?;

    module.register_method("ceph-mon-dump", |_, _| {
        //输出格式化的mon map
        Ok(mon::mon_dump())
    })?;

    module.register_method("ceph-mon-versions", |_, _| {
        Ok(mon::mon_versions())
    })?;
    
    //列出可用的mon map特性
    module.register_method("ceph-mon-feature-ls", |_, _| {
        Ok(mon::mon_feature_ls())
    })?;
    
    Ok(())
}


///pg method register
pub fn ceph_pg_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {

    //pg list
    module.register_method("ceph-pg-list", |_, _| {
        //获取mon的元数据信息
        Ok(pg::pg_list())
    })?;

    Ok(())
}

///base method register
pub fn ceph_base_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {

    //集群基础命令
    module.register_method("ceph-df", |_, _| {
        //集群使用率
        Ok(base::df())
    })?;

    module.register_method("ceph-fsid", |_, _| {
        //集群ID
        Ok(base::fsid())
    })?;
    
    module.register_method("ceph-node-ls", |_, _| {
        //list all nodes in cluster
        Ok(base::node_ls())
    })?;
    
    //ceph status
    module.register_method("ceph-status", |_, _| {
        //集群状态
        Ok(base::status())
    })?;

    //ceph version
    module.register_method("ceph-version", |_, _| {
        //集群mon组件版本
        Ok(base::version())
    })?;
    
    Ok(())
}

///auth method register
pub fn ceph_auth_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {

    //集群认证相关
    module.register_method("ceph-auth-list", |_, _| {
        //list authentication state
        Ok(auth::auth_list())
    })?;

    module.register_method("ceph-auth-get-key", |params, _| {
        //list authentication state
        let params_map = params.parse::<HashMap<String, String>>()?;
        Ok(auth::get_key(params_map.get("client_type").unwrap(),
                         params_map.get("id").unwrap()).unwrap())
    })?;

    module.register_method("ceph-auth-get", |params, _| {
        //write keyring file with requested key
        let params_map = params.parse::<HashMap<String, String>>()?;
        let result = auth::auth_get(params_map.get("client_type").unwrap(),
                                    params_map.get("id").unwrap());
        match result {
            Ok(result) => {
                let result = json!(&result).to_string();
                Ok(result)
            },
            Err(result) => {
                Ok("Error to get auth".to_string())
            },
        }
    })?;

    module.register_method("ceph-auth-add-mgr", |params, _| {
        //创建用户，生成密钥并添加任何指定的功能
        let mgr_id = params.one::<String>()?;
        let result = auth::add_mgr(&mgr_id);
        match result {
            Ok(_) => {
                Ok(true)
            },
            Err(result) => {
                error!("Add mgr auth failed. Err: {}", result);
                Ok(false)
            }
        }
    })?;

    module.register_method("ceph-auth-add-osd", |params, _| {
        //创建用户，生成密钥并添加任何指定的功能
        let osd_id = params.one::<u64>()?;
        let result = auth::add_osd(osd_id);
        match result {
            Ok(_) => {
                Ok(true)
            },
            Err(result) => {
                error!("Add osd auth failed. Err: {}", result);
                Ok(false)
            }
        }
    })?;
    
    Ok(())
}

///fs method register
pub fn ceph_fs_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {

    //集群文件系统相关
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

    //mgr metadata
    module.register_method("ceph-mgr-metadata", |_, _| {
        //mgr metadata
        Ok(mgr::mgr_metadata())
    })?;

    //mgr versions
    module.register_method("ceph-mgr-versions", |_, _| {
        //mgr versions
        Ok(mgr::mgr_versions())
    })?;

    //mgr services
    module.register_method("ceph-mgr-services", |_, _| {
        //列出MGR模块提供的服务端点
        Ok(mgr::mgr_services())
    })?;

    //mgr module ls
    module.register_method("ceph-mgr-module-ls", |_, _| {
        //列出活跃的MGR模块
        Ok(mgr::mgr_module_ls())
    })?;

    //mgr dump
    module.register_method("ceph-mgr-dump", |_, _| {
        //列出最近的mgr map状态
        Ok(mgr::mgr_dump())
    })?;
    
    Ok(())
}

///mds method register
pub fn ceph_mds_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {

    //元数据服务器组件相关
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

//osd pool相关命令
pub fn ceph_osd_pool_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    module.register_method("ceph-pool-list", |_, _| {
        //获取ceph集群存储池列表
        Ok(ceph_pool_list())
    })?;

    module.register_method("ceph-pool-list-detail", |_, _| {
        //获取ceph集群存储池列表
        let ret = pool::pool_list_detail();
        match ret {
            Ok(ret) => Ok(ret),
            Err(e) => Ok(format!("Err: {:?}", e)),
        }
    })?;

    module.register_method("ceph-pool-stats", |_, _| {
        //获取ceph集群存储池状态
        Ok(ceph_pool_stats())
    })?;

    module.register_method("ceph-pool-get", |params, _| {
        //获取ceph集群属性
        let params_map = params.parse::<HashMap<String, String>>()?;
        let result = pool::get_param(params_map.get("pool_name").unwrap(),
                                     params_map.get("option").unwrap());
        match result {
            Ok(result) => {
                Ok(result)
            },
            Err(result) => {
                Ok(format!("Error to get pool option {:?}", result))
            },
        }
    })?;

    module.register_method("ceph-pool-create", |params, _| {
        //创建存储池
        let create_pool_dto = params.parse::<pool::CreatePoolDto>()?;
        let result = pool::create(&create_pool_dto.pool,
                                  create_pool_dto.pg_num,
                                  create_pool_dto.pgp_num);
        match result {
            Ok(result) => {
                Ok(result)
            },
            Err(result) => {
                Ok(format!("Error to create pool {:?}", result))
            },
        }
    })?;


    module.register_method("ceph-pool-delete", |params, _| {
        //创建存储池
        let pool = params.one::<String>()?;
        let result = pool::delete(&pool);
        match result {
            Ok(result) => {
                Ok(result)
            },
            Err(result) => {
                Ok(format!("Error to delete pool {:?}", result))
            },
        }
    })?;

    module.register_method("ceph-pool-rename", |params, _| {
        //存储池修改名称
        let params_map = params.parse::<HashMap<String, String>>()?;
        let src_pool = params_map.get("src_pool").unwrap();
        let desc_pool = params_map.get("dest_pool").unwrap();
        let result = pool::rename(src_pool, desc_pool);
        match result {
            Ok(result) => {
                Ok(result)
            },
            Err(result) => {
                Ok(format!("Error to rename pool"))
            },
        }
    })?;

    module.register_method("ceph-pool-get-quota", |params, _| {
        //创建存储池
        let pool = params.one::<String>()?;
        let result = pool::get_quota(&pool);
        match result {
            Ok(result) => {
                Ok(result)
            },
            Err(result) => {
                Ok(format!("Error to get pool quota"))
            },
        }
    })?;

    module.register_method("ceph-pool-set-quota", |params, _| {
        //设置存储池max_objects | max_bytes
        let params_map = params.parse::<HashMap<String, String>>()?;
        let pool = params_map.get("pool").unwrap();
        let field = params_map.get("field").unwrap();
        let val = params_map.get("val").unwrap();
        let result = pool::set_quota(pool, field, val);
        match result {
            Ok(ret) => {
                Ok(ret)
            },
            Err(e) => {
                Ok(format!("Error to set pool quota max objects: {}", e.to_string()))
            },
        }
    })?;
    
    Ok(())
}

///config key method register
pub fn ceph_config_key_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {

    //config key
    module.register_method("ceph-config-key-exists", |params, _| {
        //exist
        Ok(config_key::exists(&params.one::<String>()?).unwrap())
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
