use std::process::{Output};

pub fn reflect_cmd_result(output : Output) -> String{

    if output.status.success(){
        String::from("Done")
    } else {
        String::from_utf8_lossy(&output.stderr).to_string()
    }
}