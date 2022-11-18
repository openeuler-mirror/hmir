
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");



use std::ffi::CString;



pub fn hmir_ipmi_main()
{
    let mut name = CString::new("open").unwrap();
    unsafe {
        let ipmi_main_intf = ipmi_intf_load(name.into_raw());

        ipmi_sdr_list_cache(ipmi_main_intf);
        println!("{:?}",ipmi_main_intf);
    }
}

pub fn hmir_ipmi_sdr_main()
{
    let mut name = CString::new("open").unwrap();
    unsafe {
        let intf = ipmi_intf_load(name.into_raw());
        let rc = ipmi_sdr_print_sdr(intf, 0xfe);
    }
}

#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn it_works() {

    }


}






