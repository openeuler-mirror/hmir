use crate::{ceph_client, command};

use hmir_hash::hmir_result::HmirResult;
use ceph::ceph::Rados;
use ceph::error::{RadosError};
use serde_json::json;
use crate::osd::util;

///osd树形拓扑
pub fn osd_tree() -> String {
    command::mon_exec("osd tree")
}

///osd组件版本查询
pub fn osd_versions() -> String {
    command::mon_exec("osd versions")
}

///osd组件元数据信息查询
pub fn osd_metadata() -> String {
    command::mon_exec("osd metadata")
}

///osd延迟测试接口
pub fn osd_perf() -> String {
    command::mon_exec("osd perf")
}

///crush规则查询
pub fn osd_crush_rule_dump() -> String {
    command::mon_exec("osd crush rule dump")
}


pub fn get_osd_status(client : Result<Rados, RadosError>,
                      cmd : serde_json::Value) -> Result<String, RadosError> {
    let ret = client.unwrap().ceph_mon_command_without_data(&cmd)?;
    let result = ret.1;
    match result {
        None => {
            Result::Err(RadosError::Error(String::from("Can't get osd status")))
        },
        Some(result) => {
            Result::Ok(result)
        }
    }
}


///获取osd状态
pub fn osd_status() -> HmirResult<String> {
    let client : Result<Rados, RadosError>;
    ceph_client!(client);
    let cmd = json!({
        "prefix": "osd status",
        "format": "json"
    });
    let ret = get_osd_status(client, cmd);
    match ret {
        Ok(return_data) => {
            println!("return_data : {}", return_data);
            
            //处理该请求的返回值,包含特殊字符(\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m)
            HmirResult::new(0,
                            "".to_string(),
                            util::process_osd_status_result(return_data))
        },
        Err(e) => {
            HmirResult::new(1,
                            e.to_string(),
                            "".to_string())
        }
    }

}