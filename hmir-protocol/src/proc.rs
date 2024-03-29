
use serde::{Serialize,Deserialize};

#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct ProcInfo {
    pub pid: i32,
    pub user: String,
    pub priority: i64,
    pub nice : i64,
    pub virt : u64,
    pub res : u64,
    pub shr : u64,
    pub state : String,
    pub cpu_usage : f64,
    pub mem_usage : f64,
    pub command : String,
    pub cmdline : String
}
