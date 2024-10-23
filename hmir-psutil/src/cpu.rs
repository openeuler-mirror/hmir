use psutil::cpu;
use std::{sync::Mutex};
use lazy_static::lazy_static;
use serde::*;

/*
全局静态单例,获取持续变化数据
*/
lazy_static! {
    static ref COLLECTOR : Mutex<cpu::CpuPercentCollector> = Mutex::new(cpu::CpuPercentCollector::new().unwrap());
}

/*
CPU信息集合
*/
#[derive(Deserialize, Serialize, Debug)]
pub struct CpuInfo {
    count : u64,
    percent : f32,
}

/*
返回CPU线程数
*/
pub fn count() -> u64 {
    cpu::cpu_count()
}

/*
返回CPU使用率
*/
pub fn percent() -> f32 {
    COLLECTOR.lock().unwrap().cpu_percent().unwrap()
}

pub fn stats() {
    todo!();
}

/*
获取CPU频率信息
*/
pub fn freq() -> cpu::CpuFreq {
    cpu::cpu_freq().unwrap()
}

/*
获取CPU信息集合
*/
pub fn info() -> CpuInfo {
    let info = CpuInfo {
        count : count(),
        percent : percent(),
    };
    info
}
