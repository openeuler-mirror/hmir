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
    pub static HMIR_SUCCESS: usize = 0; //成功
    pub static HMIR_ERR_COMM: usize = 1;//通用错误
    pub static HMIR_ERR_TOKEN : usize = 2;//无效的token
    pub static HMIR_ERR_USERNAME: usize = 3;//错误的用户名
    pub static HMIR_ERR_PASSWORD: usize = 4;//错误的密码
    pub static HMIR_ERR_CONNECT_SERVER : usize = 5; //无法连接服务器
    pub static HMIR_ERR_COMMAND: usize = 6;//命令执行失败

    lazy_static! {
        pub static ref HMIR_MSG: Vec<&'static str> = vec!["成功", "通用错误", "无效的token", "错误的用户名", "错误的密码","无法连接服务器","命令执行失败"];
    }
}