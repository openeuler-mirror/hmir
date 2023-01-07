use sysctl::Sysctl;

pub fn sysctl_os_type() -> String
{
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    const CTLNAME: &str = "kern.ostype";

    #[cfg(any(target_os = "linux", target_os = "android"))]
    const CTLNAME: &str = "kern.ostype";

    let ctl = sysctl::Ctl::new(CTLNAME).unwrap();
    let val = ctl.value_string().unwrap();
    return val;
}

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

    #[test]
    fn sysctl_os_type_it_works(){
        let value = sysctl_os_type();
        println!("{}",value);

    }
}