use ceph::error::RadosError;
use ceph::error::RadosResult;
use hmir_ceph::pool;
use jsonrpsee::core::middleware::{self, Headers, MethodKind, Params};
use jsonrpsee::types::error::CallError;
use std::result;
use std::collections::HashMap;

#[test]
pub fn test_pool_set_quota() {
    let mut result = pool::set_quota("test2", "max_objects","1001");
    match result {
        Ok(result) => println!("result : {:?}", result),
        Err(err) => println!("result : {:?}", err),
    }

    result = pool::set_quota("test2", "max_bytes","1001");
    match result {
        Ok(result) => println!("result : {:?}", result),
        Err(err) => println!("result : {:?}", err),
    }
}

#[test]
pub fn test_pool_get_quota() {
    let result = pool::get_quota("test2");
    match result {
        Ok(result) => println!("result : {:?}", result),
        Err(err) => println!("result : {:?}", err),
    }
}

#[test]
pub fn test_pool_list_detail() {
    let result = pool::pool_list_detail();
    println!("result : {:?}", result);
}

#[test]
pub fn test_pool_create() {
    let result = pool::create("test-pool11", 8, 8);
    match result { 
        Ok(result) => println!("result : {:?}", result),
        Err(err) => println!("result : {:?}", err),
    }
}

#[test]
pub fn test_pool_delete() {
    let result = pool::delete("test-pool11");
    match result {
        Ok(result) => println!("result : {:?}", result),
        Err(err) => println!("result : {:?}", err),
    }
}

#[test]
pub fn test_pool_rename() {
    let result = pool::rename("test1", "test2");
    match result {
        Ok(result) => println!("result : {:?}", result),
        Err(err) => println!("result : {:?}", err),
    }
}

#[test]
pub fn test_pool_rename2() -> Result<(), CallError> {
    let src_json = "{\"src_pool\":\"pool-test2\",\"dest_pool\":\"pool-test1\"}";
    let params = Params::new(Option::Some(src_json));
    let c = params.parse::<HashMap<String, String>>()?;
    println!("{:?}", c);
    println!("{:?}", c.get("src_pool").unwrap());
    println!("{:?}", c.get("dest_pool").unwrap());
    let result = pool::rename(c.get("src_pool").unwrap(), c.get("dest_pool").unwrap());
    match result {
        Ok(result) => println!("result : {:?}", result),
        Err(err) => println!("result : {:?}", err),
    }
    Ok(())
}

#[test]
pub fn test_json_parse() -> result::Result<(), CallError> {
    let src_json = "{\"pool\":\"pool-test1\",\"pg_num\":16, \"pgp_num\":16}";
    let params = Params::new(Option::Some(src_json));
    let c = params.parse::<pool::CreatePoolDto>()?;
    println!("{:?}", c);
    println!("{:?}", c.pgp_num);
    Ok(())
}

#[test]
pub fn test_json_parse_map() -> result::Result<(), CallError> {
    let src_json = "{\"src_pool\":\"pool-test2\",\"dest_pool\":\"pool-test1\"}";
    let params = Params::new(Option::Some(src_json));
    let c = params.parse::<HashMap<String, String>>()?;
    println!("{:?}", c);
    println!("{:?}", c.get("src_pool").unwrap());
    println!("{:?}", c.get("dest_pool").unwrap());
    Ok(())
}