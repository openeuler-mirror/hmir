use crate::client_instance;
use crate::ws_client_mgr::CLIENT_MAP;

#[allow(dead_code)]
pub fn ovs_query_connection(host: &str) -> (usize,String)
{
    let h = host.to_string();
    return client_instance!(&h).ovs_query_connection();
}

#[allow(dead_code)]
pub fn ovs_query_bridges(host: &str) -> (usize, String)
{
    let h = host.to_string();
    return client_instance!(&h).ovs_query_bridges();
}

#[allow(dead_code)]
pub fn ovs_query_ports(host: &str) -> (usize, String)
{
    let h = host.to_string();
    return client_instance!(&h).ovs_query_ports();
}

#[allow(dead_code)]
pub fn ovs_vsctl_add_br(host: &str, br_name: &str) -> (usize, String)
{
    let h = host.to_string();
    return client_instance!(&h).ovs_vsctl_add_br(br_name);
}

#[allow(dead_code)]
pub fn ovs_vsctl_del_br(host: &str, br_name: &str) -> (usize, String)
{
    let h = host.to_string();
    return client_instance!(&h).ovs_vsctl_del_br(br_name);
}

#[allow(dead_code)]
pub fn ovs_ofctl_forbid_dstip(host: &str, br_name: &str, dst_ip: &str, in_port:&str) -> (usize, String)
{
    let h = host.to_string();
    return client_instance!(&h).ovs_ofctl_forbid_dstip(br_name, dst_ip, in_port);
}