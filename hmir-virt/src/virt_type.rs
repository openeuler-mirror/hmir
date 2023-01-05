use serde::{Serialize};
use serde_json::json;

#[derive(Debug, Default, Clone, Serialize)]
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

#[derive(Debug, Default, Clone, Serialize)]
pub struct HmirDomain{
    pub id   : String,
    pub name : String,
    pub state: String,
    pub max_mem: String,
    pub cpus : String
}

impl HmirDomain{
    pub fn new(id:u32, name:String, state:u32, max_mem:u64, cpus:u32) -> HmirDomain{
        HmirDomain { 
            id: id.to_string(), 
            name: name, 
            state: state.to_string(), 
            max_mem: max_mem.to_string(), 
            cpus: cpus.to_string() 
        }
    }
}