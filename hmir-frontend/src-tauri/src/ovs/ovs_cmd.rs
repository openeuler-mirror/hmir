

use hmir_ws_client_mgr::net::net_mgr as clientmgr;

#[tauri::command]
pub fn cmd_ovs_query_connection(host : &str) -> (usize,String)
{
    return clientmgr::ovs_query_connection(host);
}

#[tauri::command]
pub fn cmd_ovs_query_bridges(host : &str) -> (usize,String)
{
    return clientmgr::ovs_query_bridges(host);
}

#[tauri::command]
pub fn cmd_ovs_query_ports(host: &str) -> (usize, String)
{
    return clientmgr::ovs_query_ports(host);
}

#[tauri::command]
pub fn cmd_ovs_vsctl_add_br(host: &str, br_name: &str) ->(usize, String)
{
    return clientmgr::ovs_vsctl_add_br(host, br_name);
}

#[tauri::command]
pub fn cmd_ovs_vsctl_del_br(host: &str, br_name: &str) ->(usize, String)
{
    return clientmgr::ovs_vsctl_del_br(host, br_name);
}

#[tauri::command]
pub fn cmd_ovs_ofctl_forbid_dstip(host: &str, br_name:&str, dst_ip:&str, in_port:&str) -> (usize, String)
{
    return clientmgr::ovs_ofctl_forbid_dstip(host, br_name, dst_ip, in_port);
}