use ceph::ceph::Rados;
use crate::command;
use crate::ceph_client;
use ceph::cmd;
use ceph::error::{RadosError};
use hmir_hash::hmir_result::HmirResult;

///集群使用率
pub fn df() -> String {
    command::mon_exec("df")
}

///集群ID
pub fn fsid() -> String {
    command::mon_exec("fsid")
}

///列出相关服务运行节点
pub fn node_ls() -> String {
    command::mon_exec("node ls")
}

///列出集群状态
pub fn status() -> HmirResult<String> {
    let client : Result<Rados,RadosError>;
    ceph_client!(client);
    let ret = cmd::status(&client.unwrap());
    match ret {
        Ok(result) => {
            HmirResult::new(0,
                            String::from(""),
                            result)
        },
        Err(e) => {
            HmirResult::new(1,
                            format!("Error to get ceph status, {}", e.to_string()),
                            String::from(""))
        }
    }
}

///show mon daemon version
pub fn version() -> String {
    command::mon_exec("version")
}