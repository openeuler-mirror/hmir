use std::process::Command;
use std::collections::HashMap;
use jsonrpsee::ws_server::RpcModule;

const OFCTL_CMD: &str= "ovs-ofctl";


pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{

    Ok(())
}