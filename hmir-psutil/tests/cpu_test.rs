#[cfg(test)]
mod cpu_test {
    use hmir_psutil::*;

    #[test]
    pub fn test_info() {
        let info = cpu::info();
        println!("run test {:?}", info);
    }

}
