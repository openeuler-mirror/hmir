use hmir_errno::errno;
use hmir_hash::HashWrap;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use crate::ws_client::RequestClient;

use jsonrpsee_types::ParamsSer;
use serde_json::json;
use std::collections::BTreeMap;


impl RequestClient {
    #[allow(dead_code)]
    pub fn proc_process_info(&self) -> (usize, String) {
        todo!()
    }
}