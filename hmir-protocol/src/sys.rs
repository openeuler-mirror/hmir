use serde::{Serialize,Deserialize};

#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct PciDeviceInfo {
    pub cls : String, //ID_PCI_CLASS_FROM_DATABASE
    pub model : String, //ID_MODEL_FROM_DATABASE || PCI_ID || ""
    pub vendor: String, //ID_VENDOR_FROM_DATABASE
    pub slot : String,  //PCI_SLOT_NAME
}


#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct SystemAllInfo {
    pub board_vendor : String,
    pub board_name : String,    //硬件
    pub chassis_serial : String,//资产标签
    pub machine_id : String,    //机器编号
    pub os_release : String,
    pub hostname : String,
    pub bios_vendor : String,   //LENOVO
    pub bios_version : String,  //BIOS 版本
    pub bios_date   : String,   //BIOS 日期
    pub product_name : String,
    pub product_version : String,
    pub model_name : String,
}