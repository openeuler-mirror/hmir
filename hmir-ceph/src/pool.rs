use std::borrow::Borrow;
use crate::command;
use crate::ceph_client;
use ceph::{cmd, PoolOption, error::RadosResult, error::RadosError};
// use serde_json::Value::String;
use std::string::String;
use serde_json::json;
use serde_json::*;
use serde::*;


///set object or byte limit on pool
pub fn set_quota(pool: &str, field: &str, val: &str) -> RadosResult<String> {
    let cmd = json!({
        "prefix": "osd pool set-quota",
        "pool": pool,
        "field": field,
        "val": val,
        "format": "json",
    });
    println!("cmd {}", cmd);
    let client = ceph_client::get_ceph_client()?;
    let result = client.ceph_mon_command_without_data(&cmd)?;
    println!("result {:?}", result);
    match result.1 {
        Some(res) => Ok(res),
        None => Err(RadosError::Error(format!(
            "Unable to parse osd pool set-quota output"
        ))),
    }
}

///obtain object or byte limits for pool
pub fn get_quota(pool: &str) -> RadosResult<String> {
    let cmd = json!({
        "prefix": "osd pool get-quota",
        "pool": pool,
        "format": "json",
    });
    let client = ceph_client::get_ceph_client()?;
    let result = client.ceph_mon_command_without_data(&cmd)?;
    let return_data = String::from_utf8(result.0)?;
    let mut l = return_data.lines();
    match l.next() {
        Some(res) => Ok(res.into()),
        None => Err(RadosError::Error(format!(
            "Unable to parse osd pool quota-get output: {:?}",
            return_data,
        ))),
    }
}

///存储池修改名称
pub fn rename(src_pool: &str, dest_pool: &str) -> RadosResult<String> {
    let cmd = json!({
        "prefix": "osd pool rename",
        "srcpool": src_pool,
        "destpool": dest_pool,
        "format": "json",
    });
    let client = ceph_client::get_ceph_client()?;
    let result = client.ceph_mon_command_without_data(&cmd)?;
    match result.1 {
        Some(res) => Ok(res),
        None => Err(RadosError::Error(format!(
            "Unable to parse osd pool rename output"
        ))),
    }
}


#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePoolDto {
    pub pool: String,
    pub pg_num: u64,
    pub pgp_num: u64
}

///创建存储池
pub fn create(pool_name: &str, pg: u64, pgp: u64) -> RadosResult<String> {
    let cmd = json!({
        "prefix": "osd pool create",
        "pool": pool_name,
        "pg_num": pg,
        "pgp_num": pgp,
        "format": "json",
    });
    let client = ceph_client::get_ceph_client()?;
    let result = client.ceph_mon_command_without_data(&cmd)?;
    match result.1 {
        Some(res) => Ok(res),
        None => Err(RadosError::Error(format!(
            "Unable to parse osd pool create output"
        ))),
    }
}

///删除存储池
pub fn delete(pool_name: &str) -> RadosResult<String> {
    let cmd = json!({
        "prefix": "osd pool delete",
        "pool": pool_name,
        "pool2": pool_name,
        "sure": "--yes-i-really-really-mean-it",
        // "sure": "--yes-i-really-mean-it",
        "format": "json",
    });
    let client = ceph_client::get_ceph_client()?;
    let result = client.ceph_mon_command_without_data(&cmd)?;
    match result.1 {
        Some(res) => Ok(res),
        None => Err(RadosError::Error(format!(
            "Unable to parse osd pool delete output"
        ))),
    }
}

///存储池列表
pub fn pool_list() -> String {
    command::mon_exec("osd pool ls")
}

#[test]
pub fn test_pool_list_detail() {
    let result = pool_list_detail();
    println!("result : {:?}", result);
}

///存储池详细信息列表
pub fn pool_list_detail() -> RadosResult<String> {
    let cmd = json!({
        "prefix": "osd pool ls",
        "detail": "detail",
        "format": "json",
    });
    let client = ceph_client::get_ceph_client()?;
    let result = client.ceph_mon_command_without_data(&cmd)?;
    let return_data = String::from_utf8(result.0)?; 
    let mut l = return_data.lines();
    match l.next() {
        Some(res) => Ok(res.into()),
        None => Err(RadosError::Error(format!(
            "Unable to parse osd pool ls detail output: {:?}",
            return_data,
        ))),
    }
}

///存储池状态
pub fn pool_stats() -> String {
    command::mon_exec("osd pool stats")
}

//根据pool option字符串返回具体对应的枚举类型
fn get_pool_option(option: &str) -> Option<PoolOption> {
    match option {
        "size" => Some(PoolOption::Size),
        "min_size" => Some(PoolOption::MinSize),
        "crash_replay_interval" => Some(PoolOption::CrashReplayInterval),
        "pg_num" => Some(PoolOption::PgNum),
        "pgp_num" => Some(PoolOption::PgpNum),
        "crush_rule" => Some(PoolOption::CrushRule),
        "hashpspool" => Some(PoolOption::HashPsPool),
        "nodelete" => Some(PoolOption::NoDelete),
        "nopgchange" => Some(PoolOption::NoPgChange),
        "nosizechange" => Some(PoolOption::NoSizeChange),
        "write_fadvice_dontneed" => Some(PoolOption::WriteFadviceDontNeed),
        "noscrub" => Some(PoolOption::NoScrub),
        "nodeep-scrub" => Some(PoolOption::NoDeepScrub),
        "hit_set_type" => Some(PoolOption::HitSetType),
        "hit_set_period" => Some(PoolOption::HitSetPeriod),
        "hit_set_count" => Some(PoolOption::HitSetCount),
        "hit_set_fpp" => Some(PoolOption::HitSetFpp),
        "use_gmt_hitset" => Some(PoolOption::UseGmtHitset),
        "target_max_bytes" => Some(PoolOption::TargetMaxBytes),
        "target_max_objects" => Some(PoolOption::TargetMaxObjects),
        "cache_target_dirty_ratio" => Some(PoolOption::CacheTargetDirtyRatio),
        "cache_target_dirty_high_ratio" => Some(PoolOption::CacheTargetDirtyHighRatio),
        "cache_target_full_ratio" => Some(PoolOption::CacheTargetFullRatio),
        "cache_min_flush_age" => Some(PoolOption::CacheMinFlushAge),
        "cachem_min_evict_age" => Some(PoolOption::CacheMinEvictAge),
        "auid" => Some(PoolOption::Auid),
        "min_read_recency_for_promote" => Some(PoolOption::MinReadRecencyForPromote),
        "min_write_recency_for_promote" => Some(PoolOption::MinWriteRecencyForPromte),
        "fast_read" => Some(PoolOption::FastRead),
        "hit_set_decay_rate" => Some(PoolOption::HitSetGradeDecayRate),
        "hit_set_search_last_n" => Some(PoolOption::HitSetSearchLastN),
        "scrub_min_interval" => Some(PoolOption::ScrubMinInterval),
        "scrub_max_interval" => Some(PoolOption::ScrubMaxInterval),
        "deep_scrub_interval" => Some(PoolOption::DeepScrubInterval),
        "recovery_priority" => Some(PoolOption::RecoveryPriority),
        "recovery_op_priority" => Some(PoolOption::RecoveryOpPriority),
        "scrub_priority" => Some(PoolOption::ScrubPriority),
        "compression_mode" => Some(PoolOption::CompressionMode),
        "compression_algorithm" => Some(PoolOption::CompressionAlgorithm),
        "compression_required_ratio" => Some(PoolOption::CompressionRequiredRatio),
        "compression_max_blob_size" => Some(PoolOption::CompressionMaxBlobSize),
        "compression_min_blob_size" => Some(PoolOption::CompressionMinBlobSize),
        "csum_type" => Some(PoolOption::CsumType),
        "csum_min_block" => Some(PoolOption::CsumMinBlock),
        "csum_max_block" => Some(PoolOption::CsumMaxBlock),
        "allow_ec_overwrites" => Some(PoolOption::AllocEcOverwrites),
        &_ => None
    }
}

// Query a ceph pool.
pub fn get_param(pool_name: &str, option: &str) -> RadosResult<String> {
    let client = ceph_client::get_ceph_client()?;
    let pool_option = get_pool_option(option);
    match pool_option { 
        Some(option) => cmd::osd_pool_get(&client, 
                                          pool_name, 
                                          &option),
        None => RadosResult::Ok(String::from("Option not support"))
    }
}

#[test]
pub fn test_pool_option() {
    println!("{}", get_pool_option("size").unwrap());
    println!("{}", get_pool_option("size").unwrap());
}

#[test]
pub fn test_pool_option1() {
    let mut vec_s = vec![(String::from("a"), String::from("disable")), 
                         (String::from("b"), String::from("enable"))];
    let vec = vec_s.into_iter().filter(|x| x.1.eq("disable"))
        .map(|x| x.0).collect::<Vec<String>>();
    let vec2 = vec.iter().map(String::as_str).collect::<Vec<&str>>();
    println!("{:?}", vec2);
}
