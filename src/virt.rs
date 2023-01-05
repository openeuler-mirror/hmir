//! virt模块： 通过libvirt-rust接口 实现控制以及查询虚拟机相关能力

use jsonrpsee::ws_server::RpcModule;
use hmir_virt::*;

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {
    
    register_virt_method(module)?;

    Ok(())
}