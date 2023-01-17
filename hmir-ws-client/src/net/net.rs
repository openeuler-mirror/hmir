use hmir_errno::errno;
use hmir_hash::HashWrap;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use ws_client::RequestClient;
use crate::ws_client;

use jsonrpsee_types::ParamsSer;
use serde_json::json;
use std::collections::BTreeMap;

macro_rules! ExecOvsQuery {
    ($instance:expr, $cmd:expr, $token:expr) => {
        let (state, ret_str) = ($instance).runtime.block_on(async {
            let response : Result<String, _>= ($instance).client.request($cmd, rpc_params![$token]).await;
            match response {
                Ok(result) => {
                    let p:HashWrap<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    if p.is_success() {
                        let ret_str =  p.get(&String::from("ovs_ret")).unwrap();
                        return (p.get_code(), ret_str.clone());
                    } else {
                        return (p.get_code(), p.get_err());
                    }
                },
                _=>{
                    let err_msg = format!("{} Failed!", $cmd);
                    return (errno::HMIR_ERR_COMM, err_msg);
                }
            }
        });
        return (state, ret_str)
    }
}

macro_rules! ExecVsctlOrOfctl {
    ($instance:expr, $cmd:expr, $br_info:expr) => {
        let (state, ret_str) = ($instance).runtime.block_on(async {
            let response : Result<String, _> = ($instance).client.request($cmd, Some(ParamsSer::Map($br_info))).await;
            match response {
                Ok(result) =>{
                    let p:HashWrap<String,String> = serde_json::from_str(result.as_str()).unwrap();
                    if p.is_success() {
                        let ret_str =  p.get(&String::from("ovs_ret")).unwrap();
                        return (p.get_code(), ret_str.clone());
                    } else {
                        return (p.get_code(), p.get_err());
                    }
                },
                _=> {
                    let err_msg = format!("{} Failed!", $cmd);
                    return (errno::HMIR_ERR_COMM, err_msg);
                }
            }
        });
        return (state, ret_str);
    }
}

impl RequestClient {

    #[allow(dead_code)]
    pub fn ovs_query_connection(&self) -> (usize, String) {
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-connection", token);
    }

    #[allow(dead_code)]
    pub fn ovs_query_ports(&self) -> (usize,String){
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-ports", token);
    }

    #[allow(dead_code)]
    pub fn ovs_query_bridges(&self) -> (usize,String){
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-bridges", token);
    }

    #[allow(dead_code)]
    pub fn ovs_query_interfaces(&self) -> (usize,String){
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-interfaces", token);
    }

    #[allow(dead_code)]
    pub fn ovs_query_netflow(&self) -> (usize,String) {
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-netflows", token);
    }

    #[allow(dead_code)]
    pub fn ovs_query_ipfix(&self) -> (usize,String) {
        let token = self.token.clone();
        ExecOvsQuery!(self, "ovs-query-ipfix", token);
    }

    #[allow(dead_code)]
    pub fn ovs_vsctl_add_br(&self, br_name:&str) -> (usize, String)
    {
        let token = json!(self.token.clone());
        let mut br_info = BTreeMap::new();
        br_info.insert("br_name", json!(br_name));
        br_info.insert("token", token);

        ExecVsctlOrOfctl!(self, "ovs-vsctl-add-br", br_info);
    }

    #[allow(dead_code)]
    pub fn ovs_vsctl_del_br(&self, br_name:&str) -> (usize, String)
    {
        let token = json!(self.token.clone());
        let mut br_info = BTreeMap::new();
        br_info.insert("br_name", json!(br_name));
        br_info.insert("token", token);

        ExecVsctlOrOfctl!(self, "ovs-vsctl-del-br", br_info);
    }

    #[allow(dead_code)]
    pub fn ovs_vsctl_add_port(&self, br_name:&str, port_name:&str) -> (usize, String)
    {
        let token = json!(self.token.clone());
        let mut br_info = BTreeMap::new();
        br_info.insert("br_name", json!(br_name));
        br_info.insert("port_name", json!(port_name));
        br_info.insert("token", token);

        ExecVsctlOrOfctl!(self, "ovs-vsctl-add-port", br_info);
    }

    #[allow(dead_code)]
    pub fn ovs_vsctl_del_port(&self, br_name:&str, port_name:&str) -> (usize, String)
    {
        let token = json!(self.token.clone());
        let mut br_info = BTreeMap::new();
        br_info.insert("br_name", json!(br_name));
        br_info.insert("port_name", json!(port_name));
        br_info.insert("token", token);

        ExecVsctlOrOfctl!(self, "ovs-vsctl-del-port", br_info);
    }

    #[allow(dead_code)]
    pub fn ovs_vsctl_set_netflow_rule(&self, br_name:&str, targets:&str) -> (usize, String)
    {
        let token = json!(self.token.clone());
        let mut br_info = BTreeMap::new();
        br_info.insert("br_name", json!(br_name));
        br_info.insert("targets", json!(targets));
        br_info.insert("token", token);

        ExecVsctlOrOfctl!(self, "ovs-vsctl-set-netflow-rule", br_info);
    }

    #[allow(dead_code)]
    pub fn ovs_ofctl_forbid_dstip(&self, br_name:&str, dst_ip:&str, in_port:&str) -> (usize, String)
    {
        let token = json!(self.token.clone());
        let mut br_info = BTreeMap::new();
        br_info.insert("br_name", json!(br_name));
        br_info.insert("dst_ip", json!(dst_ip));
        br_info.insert("in_port", json!(in_port));
        br_info.insert("token", token);

        ExecVsctlOrOfctl!(self, "ovs-ofctl-forbid-dstip", br_info);
    }

}