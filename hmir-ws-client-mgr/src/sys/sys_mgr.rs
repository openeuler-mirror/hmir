use crate::client_instance;
use crate::mgr_fn_define;
use crate::ws_client_mgr::CLIENT_MAP;

mgr_fn_define!(sys_os_all_info);
mgr_fn_define!(sys_list_pci_info);
mgr_fn_define!(sys_get_date);

pub fn sys_set_date(host: & str,date : String) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).sys_set_date(date);
}

pub fn sys_set_hostname(host: & str,pretty_name : String,static_name : String) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).sys_set_hostname(pretty_name,static_name);
}

