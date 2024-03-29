
#[test]
fn get_ceph_status() {
    let client = RequestClient::new(String::from(URL));
    match client {
        Ok(mut c) => {
            let state = c.ceph_status();
            // assert_eq!(state, 0)
            println!("Result : {:?}", state)
        }
        _ => {
            println!("Get client [{}] error.", URL)
        }
    }
}