#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub mod constants {
    pub static LOCKFILE: &'static str = "/var/run/hmir/hmir.lock";
}
