//! 软件包管理模块
//!
//! 支持的系统
//! - debian系
//! - euler系
//! 支持以下的请求
//! -
//!

use jsonrpsee::ws_server::{RpcModule};
// use hmir_dpkg;

#[doc(hidden)]
pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;


    // #[test]
    // fn dpkg_list_it_worked(){
    //     hmir_dpkg::dpkg_list();
    // }

}