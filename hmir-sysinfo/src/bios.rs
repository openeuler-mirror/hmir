
use std::io::{self};
use crate::{simple_line_read};


#[derive(Clone, Debug, Default, PartialEq)]
pub struct BiosRelease {
    pub bios_vendor : String,
    pub bios_release : String,
    pub bios_version : String,
    pub bios_date   : String,
}

impl BiosRelease {
    pub fn new() -> io::Result<BiosRelease> {
        let mut bios_release = Self::default();
        bios_release.bios_date = simple_line_read("/sys/class/dmi/id/bios_date").unwrap();
        bios_release.bios_release = simple_line_read("/sys/class/dmi/id/bios_release").unwrap();
        bios_release.bios_vendor = simple_line_read("/sys/class/dmi/id/bios_vendor").unwrap();
        bios_release.bios_version = simple_line_read("/sys/class/dmi/id/bios_version").unwrap();
        Ok(bios_release)
    }



}