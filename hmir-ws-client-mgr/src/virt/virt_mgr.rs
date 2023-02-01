use crate::client_instance;
use crate::ws_client_mgr::CLIENT_MAP;

#[allow(dead_code)]
pub fn virt_check_connection(host: &str) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).virt_check_connection();
}

#[allow(dead_code)]
pub fn virt_show_hypervisor(host: &str) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).virt_show_hypervisor();
}