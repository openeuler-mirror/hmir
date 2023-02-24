use serde::{Serialize,Deserialize};


#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct DebPkgInfo {
    pub package : String,
    pub status : String,
    pub priority : String,
    pub section : String,
    pub installed_size : u64,
    pub maintainer : String,
    pub architecture : String,
    pub source : String,
    pub version : String,
    pub depends : Vec<String>,
    pub description : String,
}