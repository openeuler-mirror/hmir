use serde::{Serialize};

#[derive(Debug, Default, Clone, Serialize)]
pub struct HmirHvisor{
    pub hv_type : String,
    pub hv_ver : String,
    pub is_alive: bool,
    pub is_enc: bool,
    pub is_sec:bool
}

impl HmirHvisor{
    pub fn new(hv_type:String, hv_ver:String, is_alive:bool, is_enc: bool, is_sec:bool) -> HmirHvisor{
        HmirHvisor{
            hv_type,
            hv_ver,
            is_alive,
            is_enc,
            is_sec
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct HmirDomain{
    pub id   : String,
    pub name : String,
    pub uuid: String
}

impl HmirDomain{
    pub fn new(id:u32, name:String, uuid:String) -> HmirDomain{
        HmirDomain { 
            id: id.to_string(), 
            name,  
            uuid
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

#[derive(Debug, Default, Clone, Serialize)]
pub struct HmirStoragePool{
    pub uuid: String,
    pub state: u32,
    pub capacity: u64,
    pub allocation: u64,
    pub avaliable: u64
}

impl HmirStoragePool{
    pub fn new(uuid:String, state:u32, capacity:u64, allocation:u64, avaliable:u64) -> HmirStoragePool{
        HmirStoragePool { 
            uuid, 
            state, 
            capacity, 
            allocation, 
            avaliable 
        }
    }
}
