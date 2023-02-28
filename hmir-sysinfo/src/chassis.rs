
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

lazy_static! {
        pub static ref CHASSIS_TYPE: Vec<&'static str> = vec!["undefined",
        "Other", "Unknown", "Desktop",
        "Low profile desktop",
        "Pizza box",
        "Mini tower",
        "Tower",
        "Portable",
        "Laptop",
        "Notebook",
        "Handheld",
        "Docking station",
        "All-in-one",
        "Sub-Notebook",
        "Space-saving computer",
        "Lunch box", /* 0x10 */
        "Main server chassis",
        "Expansion chassis",
        "Sub-Chassis",
        "Bus expansion chassis",
        "Peripheral chassis",
        "RAID chassis",
        "Rack mount chassis",
        "Sealed-case PC",
        "Multi-system chassis",
        "Compact PCI", /* 0x1A */
        "Advanced TCA",
        "Blade",
        "Blade enclosure",
        "Tablet",
        "Convertible",
        "Detachable", /* 0x20 */
        "IoT gateway",
        "Embedded PC",
        "Mini PC",
        "Stick PC",
    ];

}

impl ChassisInfo {
    pub fn new() -> io::Result<ChassisInfo> {
        let mut chass = Self::default();
        chass.chassis_serial = simple_line_read("/sys/class/dmi/id/chassis_serial").unwrap_or("Invalid".to_string());
        chass.chassis_vendor = simple_line_read("/sys/class/dmi/id/chassis_vendor").unwrap_or("Invalid".to_string());
        chass.chassis_asset_tag = simple_line_read("/sys/class/dmi/id/chassis_asset_tag").unwrap_or("Invalid".to_string());
        let chassis_type = simple_line_read("/sys/class/dmi/id/chassis_type").unwrap_or("0".to_string()).parse::<usize>().unwrap();
        if chassis_type < CHASSIS_TYPE.len() {
            chass.chassis_type = CHASSIS_TYPE[chassis_type].into();
        }else {
            chass.chassis_type = CHASSIS_TYPE[1].into();
        }
        chass.chassis_version = simple_line_read("/sys/class/dmi/id/chassis_version").unwrap_or("Invalid".to_string());
        Ok(chass)
    }
}