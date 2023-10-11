

use std::convert::TryFrom;
use parser::client::ClientFrame;
use tokio::net::TcpStream;


pub async  fn handle_frame(frame_buffer : Vec<u8>,stream: &TcpStream) {
    let frame =  ClientFrame::try_from(frame_buffer);
    if let Ok(ClientFrame::Send(f)) = frame {
        println!("{:?}",f);
    } else if let Ok(ClientFrame::Connect(f)) = frame {
        println!("{:?}",f);
    } else {
        println!("---------");
    }
}