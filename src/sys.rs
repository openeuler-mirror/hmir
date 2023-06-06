
use std::process;
use std::str::Split;
use regex::Regex;
use std::sync::{Mutex};
use jsonrpsee::ws_server::{RpcModule};

use hmir_sysinfo::release::OsInfo;
use hmir_sysinfo::bios::BiosRelease;
use hmir_sysinfo::chassis::ChassisInfo;
use hmir_sysinfo::machine::MachineInfo;
use hmir_sysinfo::product::ProductInfo;
use hmir_sysinfo::board::BoardInfo;
use hmir_hash::HashWrap;
use hmir_protocol::sys;
use hmir_errno::errno;
use hmir_token::TokenChecker;
use std::ffi::OsString;
use serde::{Serialize,Deserialize};
use chrono::{Local};


fn gethostname() -> OsString {
    use nix::libc::{c_char, sysconf, _SC_HOST_NAME_MAX};
    use std::os::unix::ffi::OsStringExt;
    // Get the maximum size of host names on this system, and account for the
    // trailing NUL byte.
    let hostname_max = unsafe { sysconf(_SC_HOST_NAME_MAX) };
    let mut buffer = vec![0; (hostname_max as usize) + 1];
    let returncode = unsafe { nix::libc::gethostname(buffer.as_mut_ptr() as *mut c_char, buffer.len()) };
    if returncode != 0 {
        // There are no reasonable failures, so lets panic
        panic!(
            "gethostname failed: {}
    Please report an issue to <https://github.com/swsnr/gethostname.rs/issues>!",
            std::io::Error::last_os_error()
        );
    }
    // We explicitly search for the trailing NUL byte and cap at the buffer
    // length: If the buffer's too small (which shouldn't happen since we
    // explicitly use the max hostname size above but just in case) POSIX
    // doesn't specify whether there's a NUL byte at the end, so if we didn't
    // check we might read from memory that's not ours.
    let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
    buffer.resize(end, 0);
    OsString::from_vec(buffer)
}

lazy_static! {
    static ref SYS_PCI_INFO_CACHE: Mutex<String> = Mutex::new(String::new());
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
    let mut is_pci;
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

#[cfg(target_os = "linux")]
fn sys_all_os_info() -> String
{
    let bios_info = BiosRelease::new().unwrap();
    let board_info = BoardInfo::new().unwrap();
    let chass_info = ChassisInfo::new().unwrap();
    let machine_info = MachineInfo::new().unwrap();
    let os_release = OsInfo::new().unwrap();
    let product_info = ProductInfo::new().unwrap();
    let cpu_info = procfs::CpuInfo::new().unwrap();
    let cpu_model = cpu_info.num_cores().to_string() +"x " + cpu_info.model_name(0).unwrap_or("Unkown");


    let info = sys::SystemAllInfo {
        board_vendor: board_info.board_vendor.into(),
        board_name: board_info.board_name.into(),
        chassis_serial: chass_info.chassis_serial.into(),
        chassis_type : chass_info.chassis_type.into(),
        machine_id: machine_info.machine_id.into(),
        os_release: os_release.name.into(),
        hostname: gethostname().into_string().unwrap(),
        bios_vendor: bios_info.bios_vendor.into(),
        bios_version: bios_info.bios_version.into(),
        bios_date: bios_info.bios_date.into(),
        product_name : product_info.product_name.into(),
        product_version : product_info.product_version.into(),
        model_name : cpu_model.into()
    };

    let mut map  = hmir_hash::HashWrap::new();
    map.insert("sysinfo",info);

    let serialized = serde_json::to_string(&map).unwrap();
    serialized
}

pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()> {

    module.register_method("sys-list-pci-info", |params, _| {
        //默认没有error就是成功的
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);
        Ok(sys_list_pci_info())
    })?;

    #[cfg(target_os = "linux")]
    {
        module.register_method("sys-os-all-info", |params, _| {
            //默认没有error就是成功的
            let token = params.one::<std::string::String>()?;
            TokenChecker!(token);
            Ok(sys_all_os_info())
        })?;
    }

    module.register_method("sys-set-hostname", |params, _| {

        #[derive(Clone, Debug,Serialize,Deserialize)]
        struct P {
            token:String,
            pretty_name:String,
            static_name:String,
        }

        //默认没有error就是成功的
        let param = params.parse::<P>()?;
        TokenChecker!(param.token);
        Ok(sys_set_hostname(param.pretty_name,param.static_name))
    })?;


    module.register_method("sys-set-date", |params, _| {

        #[derive(Clone, Debug,Serialize,Deserialize)]
        struct P {
            token:String,
            date:String,
        }

        //默认没有error就是成功的
        let param = params.parse::<P>()?;
        TokenChecker!(param.token);
        Ok(sys_set_date(param.date))
    })?;

    module.register_method("sys-get-date", |params, _| {
        let token = params.one::<std::string::String>()?;
        TokenChecker!(token);
        Ok(sys_get_date())
    })?;


    Ok(())
}


pub fn sys_set_hostname(pretty_hostname : String, static_hostname : String) -> String
{
    let output = process::Command::new("hostnamectl")
        .arg("set-hostname")
        .arg("--pretty")
        .arg(pretty_hostname)
        .arg("--static")
        .arg(static_hostname)
        .output()
        .expect("failed to execute process");
    let mut map  = HashWrap::<i32,i32>:: new();
    match output.status.success() {
        true => map.set_code(0),
        false => map.set_code(errno::HMIR_ERR_COMMAND),
    }

    let serialized = serde_json::to_string(&map).unwrap();
    serialized
}

fn validate_datetime_format(datetime_str : & String) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d:{2}\d{2}$").unwrap();
    return re.is_match(datetime_str);
}

pub fn sys_set_date(date: String) -> String {
    let mut map  = HashWrap::<i32,i32>:: new();

    match validate_datetime_format(&date) {
        false => {
            map.error(errno::HMIR_ERR_PARAM, errno::HMIR_MSG[errno::HMIR_ERR_PARAM].to_string());
        }
        true => {
            let output = process::Command::new("date")
                .arg("-s")
                .arg(date)
                .output()
                .expect("failed to execute process");
            match output.status.success() {
                true => map.set_code(0),
                false => map.set_code(errno::HMIR_ERR_COMMAND),
            }
        }
    }

    let serialized = serde_json::to_string(&map).unwrap();
    serialized
}

pub fn sys_get_date() -> String {
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now().format(fmt);
    now.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn format_json(json_str: &str) -> anyhow::Result<()>   {
        let v: Value = serde_json::from_str(json_str)?;
        println!("{}", serde_json::to_string_pretty(&v)?);
        Ok(())
    }

    #[test]
    fn test_sys_all_os_info() {
        let info = sys_all_os_info();
        // println!("{}",info);
        format_json(info.as_str());
    }

    #[test]
    fn test_set_hostname(){
        sys_set_hostname("I am dwj".to_string(),"dwj".to_string());
    }

    #[test]
    fn test_set_datetime() {
        let out = sys_set_date("2023-02-23 15:08:00".to_string());
        println!("{}",out);
    }

    #[test]
    fn test_get_date(){
        sys_get_date();
    }
}

