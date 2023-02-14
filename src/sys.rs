
use std::process;
use std::str::Split;
use regex::Regex;
use std::sync::{RwLock,Mutex};
use jsonrpsee::ws_server::{RpcModule};

use hmir_sysinfo::release::OsInfo;
use hmir_sysinfo::bios::BiosRelease;
use hmir_sysinfo::chassis::ChassisInfo;
use hmir_sysinfo::machine::MachineInfo;
use hmir_sysinfo::product::ProductInfo;
use hmir_hash::HashWrap;
use hmir_protocol::sys;
use hmir_errno::errno;
use hmir_token::TokenChecker;


lazy_static! {
    static ref SYS_PCI_INFO_CACHE: Mutex<String> = Mutex::new(String::new());
}

fn sys_os_info() -> String
{
    let release = OsInfo::new().unwrap();
    release.name.into()
}

fn sys_bios_info()
{

    let b = BiosRelease::new().unwrap();
    println!("vendor  {}",b.bios_vendor);
    println!("date    {}",b.bios_date);
    println!("release {}",b.bios_release);
    println!("version {}",b.bios_version);
}

fn sys_chassis_info()
{
    let b = ChassisInfo::new().unwrap();
    println!("Serial:  {}",b.chassis_serial);
    println!("Vendor:    {}",b.chassis_vendor);
    println!("Asset Tag: {}",b.chassis_asset_tag);
    println!("Type: {}",b.chassis_type);
    println!("Version: {}",b.chassis_version);
}

fn sys_machine_info()
{
    let b = MachineInfo::new().unwrap();
    println!("Machine ID:  {}",b.machine_id);
}

fn sys_product_info()
{
    let b = ProductInfo::new().unwrap();
    println!("Product is:  {:?}",b);
}

pub fn init_sysinfo()
{
    *SYS_PCI_INFO_CACHE.lock().unwrap() = sys_pci_info();
}

fn sys_pci_info() -> String
{
    /* we expect udev db paragraphs like this:
     *
       P: /devices/virtual/mem/null
       N: null
       E: DEVMODE=0666
       E: DEVNAME=/dev/null
       E: SUBSYSTEM=mem
    */
    let udev_path_re = Regex::new(r"^P: (.*)$").unwrap();
    // let udev_property_re = Regex::new(r"^E: (\w+)=(.*)$").unwrap();
    let udev_pci_re = Regex::new(r"^E: SUBSYSTEM=pci$").unwrap();
    let cls_re = Regex::new(r"^E: ID_PCI_CLASS_FROM_DATABASE=(.*)$").unwrap();
    let model_re = Regex::new(r"^E: ID_MODEL_FROM_DATABASE=(.*)$").unwrap();
    let pci_id_re = Regex::new(r"^E: PCI_ID=(.*)$").unwrap();
    let slot_re = Regex::new(r"^E: PCI_SLOT_NAME=(.*)$").unwrap();
    let vendor_re = Regex::new(r"^E: ID_VENDOR_FROM_DATABASE=(.*)$").unwrap();

    let output = process::Command::new("udevadm")
        .arg("info")
        .arg("--export-db")
        .output()
        .expect("failed to execute process");

    let udevdb = std::str::from_utf8(&output.stdout);
    let records: Split<&str> = udevdb.unwrap().split("\n\n");
    let mut cls = "";
    let mut vendor = "";
    let mut model = "";
    let mut slot = "";
    let mut pci_id = "";
    let mut is_pci = false;
    let mut map  = hmir_hash::HashWrap::new();
    let mut syspath = "";

    for record in records {
        is_pci = false;
        let rlines: Split<&str> = record.split("\n");
        for line in rlines {

            if udev_path_re.is_match(line) {
                syspath = line.trim_start_matches("P: ");
            }
            else if udev_pci_re.is_match(line) {
                is_pci = true;
            } else if cls_re.is_match(line) {
                cls = line.trim_start_matches("E: ID_PCI_CLASS_FROM_DATABASE=");
            } else if model_re.is_match(line) {
                model = line.trim_start_matches("E: ID_MODEL_FROM_DATABASE=");
            } else if pci_id_re.is_match(line) {
                pci_id = line.trim_start_matches("E: PCI_ID=");
            } else if slot_re.is_match(line) {
                slot = line.trim_start_matches("E: PCI_SLOT_NAME=");
            } else if vendor_re.is_match(line) {
                vendor = line.trim_start_matches("E: ID_VENDOR_FROM_DATABASE=");
            }
        }

        if is_pci {
            let data = sys::PciDeviceInfo {
                cls: cls.to_string(),
                model: match model.is_empty() {
                    false => model.to_string(),
                    true => pci_id.to_string()
                },
                vendor: vendor.to_string(),
                slot: slot.to_string(),
            };
            map.insert(syspath,data);
        }
    }

    let serialized = serde_json::to_string(&map).unwrap();
    serialized
}

fn sys_list_pci_info() -> String {
    let result = (*SYS_PCI_INFO_CACHE.lock().unwrap()).clone();
    result
}


pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("sys-list-pci-info", |params, _| {
        //默认没有error就是成功的
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);
        Ok(sys_list_pci_info())
    })?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_release(){
        let name = sys_os_info();
        println!("name is {}",name);
    }

    #[test]
    fn test_bios_release(){
        sys_bios_info();
    }

    #[test]
    fn test_chassis_info(){
        sys_chassis_info();
    }

    #[test]
    fn test_machine_info(){
        sys_machine_info();
    }


    #[test]
    fn test_product_info(){
        sys_product_info();
    }

    #[test]
    fn test_pci_info_worked(){
        let d = sys_pci_info();
        println!("{}",d);
    }


}