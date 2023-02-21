
use std::io::{self};
use crate::{simple_line_read};


#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoardInfo {
    pub board_name : String,
    pub board_serial : String,
    pub board_vendor : String,
    pub board_version   : String,
}

impl BoardInfo {
    pub fn new() -> io::Result<BoardInfo> {
        let mut board = Self::default();
        board.board_name = simple_line_read("/sys/class/dmi/id/board_name").unwrap_or("Invalid".to_string());
        board.board_serial = simple_line_read("/sys/class/dmi/id/board_serial").unwrap_or("Invalid".to_string());
        board.board_vendor = simple_line_read("/sys/class/dmi/id/board_vendor").unwrap_or("Invalid".to_string());
        board.board_version = simple_line_read("/sys/class/dmi/id/board_version").unwrap_or("Invalid".to_string());
        Ok(board)
    }
}