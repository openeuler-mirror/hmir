use regex::*;
use lazy_static::lazy_static;
use serde::*;
use std::collections::HashMap;

lazy_static! {
    // static ref OSD_STATUS_REGEX : Regex = Regex::new(r"\u001b\[[0-9]*[;]?[0-9]*m|[\|]*[\n]*\+[\-]*[\n]*[\|]*|[\n]+\|").unwrap();
    static ref OSD_STATUS_REGEX : Regex = Regex::new(r"\u001b\[[0-9]*[;]?[0-9]*m|[\|]*[\n]*\+[\-]*[\n]*|[\n]+\|").unwrap();
    static ref OSD_STATUS_HEAD_AND_TAIL_CENTER_VERTICAL_LINE_REGEX : Regex 
    = Regex::new(r"^\||\|$").unwrap();
    
    // static ref OSD_STATUS_OBJ_FUNC : HashMap<u8, fn(&mut OsdStatus, String) -> ()> = HashMap::new();
    // OSD_STATUS_OBJ_FUNC.insert(0, set_id);
    static ref OSD_STATUS_OBJ_FUNC : HashMap<u8, fn(&mut OsdStatus, String)> = {
        let mut m : HashMap<u8, fn(&mut OsdStatus, String)> = HashMap::new();
        m.insert(0, set_id);
        m.insert(1, set_host);
        m.insert(2, set_used);
        m.insert(3, set_avail);
        m.insert(4, set_wr_ops);
        m.insert(5, set_wr_data);
        m.insert(6, set_rd_ops);
        m.insert(7, set_rd_data);
        m.insert(8, set_state);
        m
    };
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OsdStatus {
    pub id : Option<String>,
    pub host : Option<String>,
    pub used : Option<String>,
    pub avail : Option<String>,
    pub wr_ops : Option<String>,
    pub wr_data : Option<String>,
    pub rd_ops : Option<String>,
    pub rd_data : Option<String>,
    pub state : Option<String>,
}

impl OsdStatus {
    pub fn new () -> OsdStatus {
        return OsdStatus{
            id : None,
            host : None,
            used : None,
            avail : None,
            wr_ops : None,
            wr_data : None,
            rd_ops : None,
            rd_data : None,
            state : None,
        }
    }
}

pub fn set_id(obj : &mut OsdStatus, val : String) {
    obj.id = Some(val);
}

pub fn set_host(obj : &mut OsdStatus, val : String) {
    obj.host = Some(val);
}

pub fn set_used(obj : &mut OsdStatus, val : String) {
    obj.used = Some(val);
}

pub fn set_avail(obj : &mut OsdStatus, val : String) {
    obj.avail = Some(val);
}

pub fn set_wr_ops(obj : &mut OsdStatus, val : String) {
    obj.wr_ops = Some(val);
}

pub fn set_wr_data(obj : &mut OsdStatus, val : String) {
    obj.wr_data = Some(val);
}

pub fn set_rd_ops(obj : &mut OsdStatus, val : String) {
    obj.rd_ops = Some(val);
}

pub fn set_rd_data(obj : &mut OsdStatus, val : String) {
    obj.rd_data = Some(val);
}

pub fn set_state(obj : &mut OsdStatus, val : String) {
    obj.state = Some(val);
}

///处理osd status请求的返回值,
///包含特殊字符(\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m)
pub fn process_osd_status_result(result : String) -> String {
    //去除特殊字符
    let temp_status = OSD_STATUS_REGEX.replace_all(&result, "");
    //去除首尾中竖线
    let ret = OSD_STATUS_HEAD_AND_TAIL_CENTER_VERTICAL_LINE_REGEX.replace_all(&temp_status, "");
    println!("ret: {}", ret);
    let mut split_ret = ret.split("|");
    //去除表格头
    for _ in 0..9 {
        split_ret.next();
    }
    //解析每个结构对象数据
    let mut ret_vec : Vec<OsdStatus> = Vec::new();
    loop {
        //内层循环是否遍历split_ret到结束的标志
        let mut is_done = false;
        let mut index: u8 = 0;
        let mut osd_status_entry : OsdStatus = OsdStatus::new();
        loop {
            //每个osd status对象解析9个字段
            //"[{\"id\":\"0\",\"host\":\"n1\",\"used\":\"1295M\",
            //\"avail\":\"198G\",\"wr_ops\":\"0\",\"wr_data\":\"0\",
            //\"rd_ops\":\"0\",\"rd_data\":\"0\",\"state\":\"exists,up\"}]"
            if 9 > index {
                let entry = split_ret.next();
                match entry {
                    None => {
                        is_done = true;
                        break;
                    },
                    Some(val) => {
                        OSD_STATUS_OBJ_FUNC.get(&index).unwrap()(&mut osd_status_entry,
                                                                 String::from(val.trim()));
                        index = index + 1;
                    }
                }
            } else {
                ret_vec.push(osd_status_entry);
                break;
            }
        }
        if is_done {
            break;
        }
    }
    return serde_json::to_string(&ret_vec).unwrap();
}

#[test]
pub fn osd_staus_test() {
    let osd_status_color_regex : Regex = Regex::new(r"\\u001b\[[0-9]*[;]?[0-9]*m").unwrap();
    let ret = String::from("\\u001b[1m\\u001b[1;33m1294\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30mM\\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;33m 198\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30mG\\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;30m 0\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;30m 0\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;30m 0\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;30m 0\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m | exists,up |\\n| 1 | n2 | \\u001b[1m\\u001b[1;33m1294\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30mM\\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;33m 198\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30mG\\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;30m 0\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;30m 0\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;30m 0\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;30m 0\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m | exists,up |\\n| 2 | n3 | \\u001b[1m\\u001b[1;33m1294\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30mM\\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;33m 198\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30mG\\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;30m 0\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;30m 0\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;33m 2\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m | \\u001b[1m\\u001b[1;33m 106\\u001b[0m\\u001b[0m\\u001b[1m\\u001b[1;30m \\u001b[0m\\u001b[0m");
    let result = osd_status_color_regex.replace_all(&ret, "");
    println!("ret: {}", ret);
    println!("result: {}", result);
    let mut obj_test = OsdStatus::new();
    OSD_STATUS_OBJ_FUNC.get(&0).unwrap()(&mut obj_test, String::from("test"));
    println!("obj_test: {:?}", obj_test);
}