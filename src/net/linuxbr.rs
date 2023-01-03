//! linux bridge 网桥能力
//! 网桥管理命令 brctl

use jsonrpsee::ws_server::RpcModule;
use std::collections::HashMap;

use std::process::{Output};
use std::process::Command;

const BRCTL_CMD: &str = "/usr/sbin/brctl";

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("brctl-add-br", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_add_br(br_info))
    })?;

    module.register_method("brctl-del-br", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_del_br(br_info))
    })?;

    module.register_method("brctl-add-interface", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_add_interface(br_info))
    })?;

    module.register_method("brctl-del-interface", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_del_interface(br_info))
    })?;

    module.register_method("brctl-stp-on", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_stp_on(br_info))
    })?;

    module.register_method("brctl-stp-off", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(brctl_stp_off(br_info))
    })?;

    Ok(())
}


fn brctl_add_br(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} addbr {}", BRCTL_CMD, br_name);
    
    let output = exec_rule(rule, "brctl_add_br".to_string());
    reflect_cmd_result(output)
}

fn brctl_del_br(info_map : HashMap<String, String>) -> String{
    let br_name = info_map.get("br_name").unwrap();
    let rule = format!("{} delbr {}", BRCTL_CMD, br_name);
    
    let output = exec_rule(rule, "brctl_del_br".to_string());
    reflect_cmd_result(output)
}

fn brctl_add_interface(info_map : HashMap<String, String>) -> String{

    String::new()
}

fn brctl_del_interface(info_map : HashMap<String, String>) -> String{

    String::new()
}

fn brctl_stp_on(info_map : HashMap<String, String>) -> String{

    String::new()
}

fn brctl_stp_off(info_map : HashMap<String, String>) -> String{

    String::new()
}

fn reflect_cmd_result(output : Output) -> String{

    if output.status.success(){
        String::from("Done")
    } else {
        String::from_utf8_lossy(&output.stderr).to_string()
    }
}

fn exec_rule(rule: String, cmd_name: String) -> Output{
    let output = Command::new("sh")
                        .arg("-c")
                        .arg(rule).
                        output().expect(&format!("failed to excute {}", cmd_name));

    output 
}