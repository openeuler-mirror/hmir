
use std::io::{self};
use crate::{simple_line_read};


#[derive(Clone, Debug, Default, PartialEq)]
pub struct MachineInfo {
    pub machine_id : String,
}

impl MachineInfo {
    pub fn new() -> io::Result<MachineInfo> {
        let mut machine = Self::default();
        machine.machine_id = simple_line_read("/etc/machine-id").unwrap_or("Invalid".to_string());
        Ok(machine)
    }
}