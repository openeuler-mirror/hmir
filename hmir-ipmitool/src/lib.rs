
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");



use libc::printf;
use std::ffi::CString;
use core::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;



pub fn ipmi_acquire_ipmb_address(intf : * mut ipmi_intf) -> u8
{
    unsafe {
        if (*intf).picmg_avail != 0 {
            ipmi_picmg_ipmb_address(intf)
        } else if (*intf).vita_avail !=0 {
            ipmi_vita_ipmb_address(intf)
        } else {
            0
        }
    }
}


pub fn hmir_ipmi_intf_get() -> *mut ipmi_intf
{
    let mut name = CString::new("open").unwrap();
    let lookupbit = 0x10;	/* use name-only lookup by default */
    let sol_escape_char = '~';
    let devnum = 0;
    let ai_family = libc::AF_UNSPEC;

    let IPMI_BMC_SLAVE_ADDR = 0x20;

    let mut addr : u32 = 0;
    let target_lun     = 0;

    unsafe {
        let ipmi_main_intf = ipmi_intf_load(name.into_raw());
        ipmi_oem_info_init();
        ipmi_intf_session_set_lookupbit(ipmi_main_intf, lookupbit);
        ipmi_intf_session_set_sol_escape_char(ipmi_main_intf, sol_escape_char as i8);
        (*ipmi_main_intf).devnum = devnum;
        (*ipmi_main_intf).devfile;

        (*ipmi_main_intf).ai_family = ai_family;
        (*ipmi_main_intf).my_addr = IPMI_BMC_SLAVE_ADDR;

        //here need check the open is empty


        match (*ipmi_main_intf).open {
            Some(open) => {
                if open(ipmi_main_intf) < 0 {()}
            }
            None => {}
        }

        if ipmi_oem_active(ipmi_main_intf, "i82571spt".as_ptr()  as *const c_char)  == 0  {
            /*
             * Attempt picmg/vita discovery of the actual interface
             * address, unless the users specified an address.
             * Address specification always overrides discovery
             */
            if picmg_discover(ipmi_main_intf) == 0 {
                (*ipmi_main_intf).picmg_avail = 1;
            } else if vita_discover(ipmi_main_intf) == 0 {
                (*ipmi_main_intf).vita_avail = 1;
            }
        }

        if ipmi_oem_active(ipmi_main_intf, "i82571spt".as_ptr()  as *const c_char)  == 0  {
            addr = ipmi_acquire_ipmb_address(ipmi_main_intf) as u32;
        }

        if  addr != 0 && addr != (*ipmi_main_intf).my_addr {

            match (*ipmi_main_intf).set_my_addr {
                Some(fn_set_my_addr) => {
                    fn_set_my_addr(ipmi_main_intf,addr as u8);
                } ,
                None => {}
            }

            /* set local address */
            (*ipmi_main_intf).my_addr = addr;
        }

        (*ipmi_main_intf).target_addr = (*ipmi_main_intf).my_addr;

        (*ipmi_main_intf).target_lun = target_lun ;


        ipmi_main_intf
    }
}

pub fn hmir_ipmi_cleanup(intf: * mut ipmi_intf)
{
    unsafe {
        ipmi_cleanup(intf);
    }
}


pub fn hmir_ipmi_sdr_main () -> i32
{
    unsafe {
        let intf = hmir_ipmi_intf_get();
        let rc = ipmi_sdr_print_sdr(intf, 0xfe);
        hmir_ipmi_cleanup(intf);
        rc
    }
}


pub fn ipmi_sensor_print_fc_discrete()
{



}





pub fn hmir_ipmi_sensor_list() -> i32
{
    let mut name = CString::new("open").unwrap();

    unsafe {
        let intf = hmir_ipmi_intf_get();
        let itr = ipmi_sdr_start(intf, 0);
        if itr.is_null() {
            return -1;
        }

        loop {
            let header = ipmi_sdr_get_next_header(intf, itr);
            if header.is_null() {
                break;
            }

            let rec = ipmi_sdr_get_record(intf, header, itr);
            if rec.is_null() {
                let header = ipmi_sdr_get_next_header(intf, itr);
                continue;
            }

            let sdr_record_type = (*header).type_;
            match sdr_record_type {
                0x01 | 0x2 => {
                    let sensor = rec as * mut sdr_record_common_sensor;

                    let event_type = (*sensor).event_type;
                    let mut thresh_available = 1;

                    match event_type {
                        1 => {
                            let sr = ipmi_sdr_read_sensor_value(intf, sensor, sdr_record_type, 3);

                            let thresh_status = "ns";

                            let sensor_num = (*sensor).keys.sensor_num;
                            let owner_id = (*sensor).keys.owner_id;
                            let lun = (*sensor).keys._bitfield_1.get(0,2) as u8;
                            let channel = (*sensor).keys._bitfield_1.get(4,4) as u8;
                            // let thresh_status = ipmi_sdr_get_thresh_status(sr, "ns");

                            let rsp = ipmi_sdr_get_sensor_thresholds(intf, sensor_num, owner_id, lun, channel);
                            if (*rsp).ccode !=0  || (*rsp).data_len != 0 {
                                thresh_available = 0;
                            }
                            // dump_sensor_fc_thredshold_csv(thresh_available, rsp, sr);

                            println!("{:?}",*sr);

                        },
                        _ => {
                            ipmi_sensor_print_fc_discrete();
                        }
                    }
                },
                _ => {  break ;}
            }

            //unsafe不进行所有权的检查，因此这里需要进行主动的释放
            libc::free(rec as *mut c_void);
        }
        ipmi_sdr_end(itr);
        hmir_ipmi_cleanup(intf);
    }
    0


}


#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn it_works() {

    }


}






