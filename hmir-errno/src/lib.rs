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


pub mod errno {
    use lazy_static::lazy_static;
    pub static HMIR_SUCCESS: i32 = 0; //成功
    pub static HMIR_COMM_ERROR: i32 = -1;//通用错误
    pub static HMIR_ERR_TOKEN : i32 = -2;//无效的token
    pub static HMIR_ERR_USERNAME: i32 = -3;//错误的用户名
    pub static HMIR_ERR_PASSWORD: i32 = -4;//错误的密码

    lazy_static! {
        pub static ref HMIR_MSG: Vec<&'static str> = vec!["成功", "通用错误", "无效的token", "错误的用户名", "错误的密码"];
    }
}