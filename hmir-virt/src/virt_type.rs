use serde::{Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize)]
pub struct HmirHvisor{
    pub hv_type : String,
    pub hv_ver : String
}

impl HmirHvisor{
    pub fn new(hv_type:String, hv_ver:String) -> HmirHvisor{
        HmirHvisor{
            hv_type: hv_type,
            hv_ver : hv_ver,
        }
    }
}