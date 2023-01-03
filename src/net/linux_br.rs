//! linux bridge 网桥能力
//! 网桥管理命令 brctl

use jsonrpsee::ws_server::RpcModule;
use std::collections::HashMap;

const BRCTL_CMD: &str = "/usr/sbin/brctl";

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("brctl-add-br", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_add_br(br_info))
    })?;

    Ok(())
}


fn brctl_add_br(info_map : HashMap<String, String>) -> String{

    String::new()
}