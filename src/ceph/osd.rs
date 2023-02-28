use jsonrpsee::ws_server::{RpcModule};
use hmir_ceph::osd::osd;

pub fn ceph_osd_register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
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

    module.register_method("ceph-osd-crush-rule-dump", |_, _| {
        //获取osd的crush规则
        Ok(ceph_osd_crush_rule_dump())
    })?;

    module.register_method("ceph-osd-status", |_, _| {
        //获取osd的状态
        Ok(osd::osd_status())
    })?;
    
    Ok(())
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