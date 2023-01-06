use serde::{Serialize};

#[derive(Debug, Default, Clone, Serialize)]
pub struct HmirHvisor{
    pub hv_type : String,
    pub hv_ver : String
}

impl HmirHvisor{
    pub fn new(hv_type:String, hv_ver:String) -> HmirHvisor{
        HmirHvisor{
            hv_type,
            hv_ver,
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
            name, 
            state: state.to_string(), 
            max_mem: max_mem.to_string(), 
            cpus: cpus.to_string() 
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct HmirNwfilter{
    pub name : String,
    pub uuid : String
}

impl HmirNwfilter{
    pub fn new(name:String, uuid:String) -> HmirNwfilter{
        HmirNwfilter {  
            name, 
            uuid
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct HmirNetwork{
    pub name : String,
    pub uuid : String,
    pub bridge: String,
    pub is_active: bool,
    pub is_persist: bool    
}

impl HmirNetwork {
    pub fn new(name:String, uuid:String, bridge:String, is_active:bool, is_persist:bool) -> HmirNetwork{
        HmirNetwork { 
            name, 
            uuid, 
            bridge, 
            is_active, 
            is_persist 
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct HmirInterface{
    pub name: String,
    pub mac: String,
    pub is_active: bool
}

impl HmirInterface {
    pub fn new(name:String, mac:String, is_active:bool) -> HmirInterface{
        HmirInterface { 
            name, 
            mac, 
            is_active 
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct HmirSecret{
    pub uuid: String,
    pub usage: String,
    pub usage_id: u32
}

impl HmirSecret {
    pub fn new( uuid:String, usage:String, usage_id:u32) -> HmirSecret{
        HmirSecret { 
            uuid, 
            usage,
            usage_id
        }
    }
}
