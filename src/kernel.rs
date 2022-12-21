//! CEPH管理模块
//!
//!

use jsonrpsee::ws_server::{RpcModule};
use hmir_hash::HashWrap;
use serde::{Deserialize, Serialize};
use std::process::Command;


macro_rules! kernel_default_result {
    ($i:expr) =>{
        let mut response = HashWrap::<i32,i32>:: new();
        response.set_code($i);
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}

#[derive(Debug, Clone,Serialize)]
pub struct ModuleInfo {
    /// The name of the module
    pub name: String,

    /// The size of the module
    pub size: u32,

    /// The number of references in the kernel to this module.  This can be -1 if the module is unloading
    pub refcount: i32,

    /// A list of modules that depend on this module.
    pub used_by: Vec<String>,

    /// The module state
    ///
    /// This will probably always be "Live", but it could also be either "Unloading" or "Loading"
    pub state: String,
}

pub fn kernel_lsmod() -> std::string::String
{
    let mut map = HashWrap::<std::string::String,ModuleInfo>:: new();

    let modules = procfs::modules().unwrap();

    for module in modules.values() {
        let m = ModuleInfo {
            name: module.name.clone(),
            size: module.size,
            refcount:module.refcount,
            used_by:module.used_by.clone(),
            state:module.state.clone()
        };
        map.insert(module.name.clone(),m);

        // println!("{:?}",module);
    }

    let serialized = serde_json::to_string(&map).unwrap();
    serialized
}

pub fn kernel_rmmod(module : std::string::String) -> std::string::String
{
    let status = Command::new("rmmod")
        .arg(module)
        .status()
        .expect("rmmod failed process");
    if status.success() {
        kernel_default_result!(0);
    }

    kernel_default_result!(-1);
}

pub fn kernel_modprobe(module : std::string::String) -> std::string::String
{
    let status = Command::new("modprobe")
        .arg(module)
        .status()
        .expect("modprobe failed process");
    if status.success() {
        kernel_default_result!(0);
    }
    kernel_default_result!(-1);
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lsmod_it_works() {
        let s = kernel_lsmod();
        println!("{}",s);
    }

}

#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("kernel-lsmod", |_, _| {
        //默认没有error就是成功的
        Ok(kernel_lsmod())
    })?;

    module.register_method("kernel-rmmod", |params, _| {
        //默认没有error就是成功的
        let module = params.one::<std::string::String>()?;
        Ok(kernel_rmmod(module))
    })?;

    module.register_method("kernel-modprobe", |params, _| {
        //默认没有error就是成功的
        let module = params.one::<std::string::String>()?;
        Ok(kernel_modprobe(module))
    })?;

    Ok(())
}

