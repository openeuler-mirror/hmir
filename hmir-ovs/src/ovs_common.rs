use std::process::{Output};
use std::process::Command;
use hmir_errno::errno;
use hmir_hash::HashWrap;

#[macro_export]
macro_rules! ExecOvsQueryResult {
    ($i:expr, $j:expr, $k:expr) => {
        let mut response = HashWrap::<String,String>:: new();
        if $i == 0 {
            response.insert(String::from("ovs_ret"), $j);
        }

        if $i !=0 {
            response.error($i, $k);
        } 
        response.set_code($i);
        let serialized = serde_json::to_string(&response).unwrap();
        return serialized;
    }
}

#[macro_export]
macro_rules! OvsTokenChecker {
    ($br_info:expr) => {
        let token_exception = json!(String::from(""));
        let token = ($br_info).get("token").unwrap_or(&token_exception).to_string();
        TokenChecker!(token);
    }
}

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

pub fn test_ovs_ret(str_in_json: String) -> bool{
    let p:HashWrap<String,String> = serde_json::from_str(str_in_json.as_str()).unwrap();
    p.is_success()
}

pub fn test_get_ret_str(str_in_json: String) -> String{
    let p:HashWrap<String,String> = serde_json::from_str(str_in_json.as_str()).unwrap();
    if p.is_success() {
        return p.get(&String::from("ovs_ret")).unwrap().clone();
    } else {
        String::from("")
    }
}

pub fn reflect_cmd_result(output : Output) -> String{

    if output.status.success(){
        ExecOvsQueryResult!(errno::HMIR_SUCCESS, "Done".to_string(), "".to_string());
    } else {
        let err_str = String::from_utf8_lossy(&output.stderr).to_string();
        ExecOvsQueryResult!(errno::HMIR_ERR_COMM, "".to_string(), err_str);
    }
}

pub fn exec_rule(rule: String, cmd_name: String) -> Output{
    let output = Command::new("sh")
                        .arg("-c")
                        .arg(rule).
                        output().expect(&format!("failed to excute {}", cmd_name));

    output 
}