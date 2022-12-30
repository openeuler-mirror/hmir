use serde::{Serialize};

// ovs port definition
#[derive(Debug, Clone, Serialize)]
pub struct OvsPort{
    pub name : String,
    pub uuid : String,
    pub mode : OvsPortMode
}

#[derive(Debug, Clone, Serialize)]
pub enum OvsPortMode{
    Access(u16),
    Trunk(Vec<u16>)
}

impl OvsPort{
    pub fn new(name:&str, uuid:&str, mode:&OvsPortMode) -> OvsPort{
        OvsPort{
            name: name.to_string(),
            uuid : uuid.to_string(),
            mode : mode.clone()
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OvsInterface{
    pub name : String,
    pub uuid : String,   
    pub mac  : String 
}

impl OvsInterface{
    pub fn new(name:&str, uuid:&str, mac:&str) -> OvsInterface{
        OvsInterface{
            name: name.to_string(),
            uuid : uuid.to_string(),
            mac : mac.to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OvsNetflow{
    pub uuid : String,   
    pub targets : String 
}

impl OvsNetflow{
    pub fn new(uuid:&str, targets:&str) -> OvsNetflow{
        OvsNetflow{
            uuid : uuid.to_string(),
            targets : targets.to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OvsIpfix{
    pub uuid : String,   
    pub targets : String 
}

impl OvsIpfix{
    pub fn new(uuid:&str, targets:&str) -> OvsIpfix{
        OvsIpfix{
            uuid : uuid.to_string(),
            targets : targets.to_string()
        }
    }
}
