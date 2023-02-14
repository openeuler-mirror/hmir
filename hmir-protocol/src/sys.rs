use serde::{Serialize,Deserialize};

#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct PciDeviceInfo {
    pub cls : String, //ID_PCI_CLASS_FROM_DATABASE
    pub model : String, //ID_MODEL_FROM_DATABASE || PCI_ID || ""
    pub vendor: String, //ID_VENDOR_FROM_DATABASE
    pub slot : String,  //PCI_SLOT_NAME
}
