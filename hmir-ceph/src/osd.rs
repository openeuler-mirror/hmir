use crate::command;

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