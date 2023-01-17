use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct HmirResult<T> {
    pub code : usize,
    pub errmsg: String,
    pub result : T,
}

impl<T> HmirResult<T> {
    pub fn new(code : usize, errmsg : String, result : T) -> HmirResult<T> {
        return HmirResult {code, errmsg, result};
    }

    pub fn is_success(&self) -> bool {
        0 == self.code
    }

    pub fn code(&self) -> usize {
        self.code
    }
}