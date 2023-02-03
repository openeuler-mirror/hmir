
use serde::{Serialize,Deserialize};

#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct ProcInfo {
    pub pid: i32,
    pub user: String,
    pub priority: i64,
    pub nice : u64,
    pub virt : u64,
    pub res : u64,
    pub sha : u64,
    pub state : String,
    pub command : String,
    pub cmdline : String
}
