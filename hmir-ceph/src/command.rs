use crate::arg;
use crate::ceph_client;
use serde_json::json;
use log::error;

///ceph mon命令执行接口封装。
///显式构造 json 命令对象并走 ceph_mon_command_without_data，
///避免 ceph_mon_command 内部用 format! 拼接 JSON 不转义、
///且对非 UTF-8 输出直接 unwrap panic 的问题。
///返回值优先取 outbuf（命令数据），数据为空时回退到 outs（状态文本）。
pub fn mon_exec(cmd: &str) -> String {
    let json_cmd = json!({
        "prefix": cmd,
        "format": "json",
    });
    let client = ceph_client::get_ceph_client();
    match client {
        Ok(client) => {
            match client.ceph_mon_command_without_data(&json_cmd) {
                Ok((outbuf, outs)) => {
                    if !outbuf.is_empty() {
                        match String::from_utf8(outbuf) {
                            Ok(data) => return data,
                            Err(e) => {
                                error!("mon_exec [{}]: outbuf is not valid UTF-8: {}", cmd, e);
                                return String::new();
                            }
                        }
                    }
                    return outs.unwrap_or_default();
                },
                Err(e) => {
                    error!("mon_exec [{}] failed: {:?}", cmd, e);
                    format!("Exec {} failed: {:?}", cmd, e)
                }
            }
        },
        Err(_e) => {
            format!("{}", arg::CONNECT_FAILED)
        }
    }
}