use ws_client::RequestClient;
use crate::ws_client;
use hmir_errno::errno;

const URL : &str = "127.0.0.1:5899";

#[test]
fn ovs_query_connection_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state ,_) = c.ovs_query_connection();
            assert_eq!(state, errno::HMIR_SUCCESS);
        }
        _ => {}
    }
} 

#[test]
fn ovs_query_ports_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state ,_)= c.ovs_query_ports();
            assert_eq!(state, errno::HMIR_SUCCESS);
        }
        _ => {}
    }
} 

#[test]
fn ovs_query_bridges_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state,_) = c.ovs_query_bridges();
            assert_eq!(state, errno::HMIR_SUCCESS);
        }
        _ => {}
    }
} 

#[test]
fn ovs_query_interfaces_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state,_) = c.ovs_query_interfaces();
            assert_eq!(state, errno::HMIR_SUCCESS)
        }
        _ => {}
    }
} 

#[test]
fn ovs_query_netflow_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state,_)= c.ovs_query_netflow();
            assert_eq!(state, errno::HMIR_SUCCESS)
        }
        _ => {}
    }
} 


#[test]
fn ovs_query_ipfix_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state,_) = c.ovs_query_ipfix();
            assert_eq!(state, errno::HMIR_SUCCESS)
        }
        _ => {}
    }
} 

#[test]
fn ovs_vsctl_add_br_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state ,_) = c.ovs_vsctl_add_br("br-ckxu");
            assert_eq!(state, errno::HMIR_SUCCESS)
        }
        _ => {}
    }
} 

#[test]
fn ovs_vsctl_del_br_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state ,_) = c.ovs_vsctl_add_br("br-ckxu");
            assert_eq!(state, errno::HMIR_SUCCESS);
            let (del_state,_) = c.ovs_vsctl_del_br("br-ckxu");
            assert_eq!(del_state, errno::HMIR_SUCCESS)
        }
        _ => {}
    }
} 

#[test]
fn ovs_vsctl_add_port_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state ,_) = c.ovs_vsctl_add_br("br-ckxu");
            assert_eq!(state, errno::HMIR_SUCCESS);
            let (port_state,_) = c.ovs_vsctl_add_port("br-ckxu", "pt-test");
            assert_eq!(port_state, errno::HMIR_SUCCESS);
        }
        _ => {}
    }
} 

#[test]
fn ovs_vsctl_del_port_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state ,_) = c.ovs_vsctl_add_br("br-ckxu");
            assert_eq!(state, errno::HMIR_SUCCESS);
            let (port_state,_) = c.ovs_vsctl_add_port("br-ckxu", "pt-test");
            assert_eq!(port_state, errno::HMIR_SUCCESS);
            let (del_br_state,_) =  c.ovs_vsctl_del_port("br-ckxu", "pt-test");
            assert_eq!(del_br_state, errno::HMIR_SUCCESS);
        }
        _ => {}
    }
} 

#[test]
fn ovs_vsctl_set_netflow_rule_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state ,_) = c.ovs_vsctl_add_br("br-ckxu");
            assert_eq!(state, errno::HMIR_SUCCESS);
            let (port_state,_) = c.ovs_vsctl_set_netflow_rule("br-ckxu", "172.30.24.92:8080");
            assert_eq!(port_state, errno::HMIR_SUCCESS);
        }
        _ => {}
    }
}

#[test]
fn ovs_ofctl_forbid_dstip_worked(){
    let client = RequestClient::new(String::from(URL));
    assert_eq!(client.is_ok(), true);
    match client {
        Ok(c) => {
            let (state ,_) = c.ovs_vsctl_add_br("br-ckxu");
            assert_eq!(state, errno::HMIR_SUCCESS);
            let (port_state,_) = c.ovs_ofctl_forbid_dstip("br-ckxu", "172.30.24.92", "br-ckxu");
            assert_eq!(port_state, errno::HMIR_SUCCESS);
        }
        _ => {}
    }
} 