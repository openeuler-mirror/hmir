
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");



use std::ffi::CString;



pub fn hmir_ipmi_main()
{
    println!("hello");

    let mut name = CString::new("open").unwrap();
    unsafe {
        let ipmi_main_intf = ipmi_intf_load(name.into_raw());
        println!("{:?}",ipmi_main_intf);
    }
}

#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn it_works() {

    }


}






