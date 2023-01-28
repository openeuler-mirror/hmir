
use serde::{Serialize,Deserialize};


#[derive(Clone, Debug, PartialEq,Serialize,Deserialize)]
pub enum UnitActiveStateType {
    Active,
    Reloading,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Other(String),
}

#[derive(Clone, Debug, PartialEq,Serialize,Deserialize)]
pub enum UnitSubStateType {
    Abandon,
    Activating,
    ActivatingDone,
    Active,
    AutoRestart,
    Dead,
    Deactivating,
    DeactivatingSigterm,
    DeactivatingSigkill,
    Elapsed,
    Exited,
    Failed,
    FinalSigterm,
    FinalSigkill,
    Mounting,
    MountingDone,
    Mounted,
    Plugged,
    Listening,
    Reload,
    Remounting,
    RemountingSigterm,
    RemountingSigkill,
    Running,
    Start,
    StartChown,
    StartPre,
    StartPost,
    Stop,
    StopPost,
    StopSigabrt,
    StopSigterm,
    StopSigkill,
    Tentative,
    Unmounting,
    UnmountingSigterm,
    UnmountingSigkill,
    Waiting,
    Other(String),
}

#[derive(Clone, Debug, PartialEq,Serialize,Deserialize)]
pub enum UnitLoadStateType {
    Stub,
    Loaded,
    NotFound,
    Error,
    Merged,
    Masked,
    Other(String),
}


#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct HmirUnit {
    pub name: String,
    pub description: String,
    pub load_state: UnitLoadStateType,
    pub active_state: UnitActiveStateType,
    pub sub_state: UnitSubStateType,
    pub follow_unit: Option<String>,
    pub object_path: String,
    pub job_id: u32,
    pub job_ty: String,
    pub job_object_path: String,
    pub requires : Vec<String>,
    pub wants : Vec<String>,
    pub wantedby : Vec<String>,
    pub conflicts : Vec<String>,
    pub before : Vec<String>,
    pub after  : Vec<String>
    //Wants
}