use jsonrpsee::ws_server::RpcModule;
use std::collections::HashMap;
use hmir_ovs::*;

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>{
    module.register_method("ovs-check-connection", |_, _| {
        Ok(ovs_query::check_connection())
    })?;
    
    module.register_method("ovs-get-ports", |_, _| {
        Ok(ovs_query::get_ports())
    })?;

    module.register_method("ovs-get-bridges", |_, _| {
        Ok(ovs_query::get_bridges())
    })?;

    module.register_method("ovs-add-port", |params, _| {
        let port_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_query::add_port(port_info))
    })?;

    module.register_method("ovs-del-port", |params, _| {
        let port_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_query::del_port(port_info))
    })?;

    module.register_method("ovs-add-br", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl::add_br(br_info))
    })?;

    module.register_method("ovs-del-br", |params, _| {
        let br_info = params.parse::<HashMap<String, String>>()?;
        Ok(ovs_vsctl::del_br(br_info))
    })?;

    Ok(())
}