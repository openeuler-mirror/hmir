// use hmir_psutil::cpu::*;

use hmir_psutil::*;
use std::{thread, time};

pub fn main() {
    println!("A");

    println!("cpu count = {}", cpu::count());
    let ten = time::Duration::from_millis(1000);
    cpu::percent();
    thread::sleep(ten);
    println!("cpu percent = {:.5}", cpu::percent());
    thread::sleep(ten);
    println!("cpu percent = {:.5}", cpu::percent());
    thread::sleep(ten);
    let info = cpu::info();
    println!("run test {:?}", info);
}
