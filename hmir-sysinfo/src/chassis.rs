
use std::io::{self};
use crate::{simple_line_read};


#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChassisInfo {
    pub chassis_serial : String,
    pub chassis_vendor : String,
    pub chassis_asset_tag : String,
    pub chassis_type   : String,
    pub chassis_version : String,
}

impl ChassisInfo {
    pub fn new() -> io::Result<ChassisInfo> {
        let mut chass = Self::default();
        chass.chassis_serial = simple_line_read("/sys/class/dmi/id/chassis_serial").unwrap_or("Invalid".to_string());
        chass.chassis_vendor = simple_line_read("/sys/class/dmi/id/chassis_vendor").unwrap_or("Invalid".to_string());
        chass.chassis_asset_tag = simple_line_read("/sys/class/dmi/id/chassis_asset_tag").unwrap_or("Invalid".to_string());
        chass.chassis_type = simple_line_read("/sys/class/dmi/id/chassis_type").unwrap_or("Invalid".to_string());
        chass.chassis_version = simple_line_read("/sys/class/dmi/id/chassis_version").unwrap_or("Invalid".to_string());
        Ok(chass)
    }



}