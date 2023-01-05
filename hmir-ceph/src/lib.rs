pub mod ceph_client;
pub mod arg;
pub mod pool;
pub mod osd;
pub mod mon;
pub mod pg;
pub mod base;
pub mod auth;
pub mod fs;
pub mod mgr;
pub mod mds;
pub mod command;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
