use std::process::{Output};
use std::process::Command;
pub fn reflect_cmd_result(output : Output) -> String{

    if output.status.success(){
        String::from("Done")
    } else {
        String::from_utf8_lossy(&output.stderr).to_string()
    }
}

pub fn exec_rule(rule: String, cmd_name: String) -> String{
    let output = Command::new("sh")
                        .arg("-c")
                        .arg(rule).
                        output().expect(&format!("failed to excute {}", cmd_name));
    
    reflect_cmd_result(output) 
}