
use std::io::{self};
use crate::{simple_line_read};


#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProductInfo {
    pub product_family: String,
    pub product_name: String,
    pub product_serial: String,
    pub product_sku: String,
    pub product_uuid: String,
    pub product_version: String,
    pub sys_vendor : String,
}

impl ProductInfo {
    pub fn new() -> io::Result<ProductInfo> {
        let mut p = Self::default();
        p.product_family = simple_line_read("/sys/class/dmi/id/product_family").unwrap_or("Invalid".to_string());
        p.product_name = simple_line_read("/sys/class/dmi/id/product_name").unwrap_or("Invalid".to_string());
        p.product_serial = simple_line_read("/sys/class/dmi/id/product_serial").unwrap_or("Invalid".to_string());
        p.product_sku = simple_line_read("/sys/class/dmi/id/product_sku").unwrap_or("Invalid".to_string());
        p.product_uuid = simple_line_read("/sys/class/dmi/id/product_uuid").unwrap_or("Invalid".to_string());
        p.product_version = simple_line_read("/sys/class/dmi/id/product_version").unwrap_or("Invalid".to_string());
        p.sys_vendor = simple_line_read("/sys/class/dmi/id/sys_vendor").unwrap_or("Invalid".to_string());
        Ok(p)
    }
}