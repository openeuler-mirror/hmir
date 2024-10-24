//! 操作系统统计
//!
//!
use jsonrpsee::ws_server::{RpcModule};
use std::collections::HashMap;
use serde_json::json;
use log::{error};
use hmir_psutil::*;

#[doc(hidden)]
pub fn register_method(module : & mut RpcModule<()>) -> anyhow::Result<()> {
    module.register_method("cpu-info", |_, _| {
        //获取CPU信息
        Ok(cpu::info())
    })?;


    Ok(())
}