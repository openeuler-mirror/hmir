use std::process::{Output};
use std::process::Command;

pub const BR_FOR_TEST: &str =  "ovs_test_br";
pub const PORT_FOR_TEST: &str = "ovs_test_port";

pub fn test_setup_env() {
    let rule_add_br = format!("ovs-vsctl add-br {}", BR_FOR_TEST);
    exec_rule(rule_add_br, "test_setup_env_br".to_string());

    let rule_chmod_ofctl = format!("sudo chmod 777 /var/run/openvswitch/{}.mgmt", BR_FOR_TEST);
    exec_rule(rule_chmod_ofctl, "test_setup_chmod_ofctl".to_string());
}

pub fn test_clear_env() {
    let rule = format!("ovs-vsctl del-br {}", BR_FOR_TEST);
    exec_rule(rule, "test_clear_env".to_string());
}

pub fn reflect_cmd_result(output : Output) -> String{

    if output.status.success(){
        String::from("Done")
    } else {
        String::from_utf8_lossy(&output.stderr).to_string()
    }
}

pub fn exec_rule(rule: String, cmd_name: String) -> Output{
    let output = Command::new("sh")
                        .arg("-c")
                        .arg(rule).
                        output().expect(&format!("failed to excute {}", cmd_name));

    output 
}