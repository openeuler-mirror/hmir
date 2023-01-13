use ceph::error::RadosError;
use ceph::error::RadosResult;
use hmir_ceph::pool;
use jsonrpsee::core::middleware::{self, Headers, MethodKind, Params};
use jsonrpsee::types::error::CallError;
use std::result;

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
pub fn test_json_parse() -> result::Result<(), CallError> {
    let src_json = "{\"pool\":\"pool-test1\",\"pg_num\":16, \"pgp_num\":16}";
    let params = Params::new(Option::Some(src_json));
    let c = params.parse::<pool::CreatePoolDto>()?;
    println!("{:?}", c);
    println!("{:?}", c.pgp_num);
    Ok(())
}