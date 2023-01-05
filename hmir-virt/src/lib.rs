//! - check-virt-connection: virt 检查虚拟机连接
//! 请求格式：
//! { 
//!     "jsonrpc":"2.0", 
//!     "id":1, 
//!     "method":"virt-check-connection",
//!     "params": {"uri":"qemu:///system"} 
//! }


use jsonrpsee::ws_server::RpcModule;
use std::collections::HashMap;


use  virt::connect::Connect;

pub fn register_virt_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("virt-check-connection", |params, _| {
        let info = params.parse::<HashMap<String, String>>()?;
        Ok(virt_check_connection(info))
    })?;

    Ok(())
}

fn virt_check_connection(info: HashMap<String, String>) -> String{
    let con_uri = info.get("uri").unwrap();
    
    match Connect::open(&con_uri) {
        Ok(mut c) => {
            match c.close() {
                Ok(_) => {"connection finish".to_string()},
                Err(e) => panic!("failed to close, code: {}, message: {}", e.code, e.message),
            }
        },
        Err(err) => panic!("Not connected, code: {}, message: {}", err.code, err.message), 
    }
}