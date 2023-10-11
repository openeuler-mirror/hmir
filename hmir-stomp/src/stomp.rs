





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


mod test {

    use super::*;

    #[test]
    fn stomp_connect_frame_test() {
        let message = b"CONNECT\n\
                accept-version:1.2\n\
                heart-beat:0,5000\n\
                \n\
                \x00"
            .to_vec();

        if let Ok(ClientFrame::Connect(frame)) = ClientFrame::try_from(message) {
            println!("{:?}",frame);
        } else {
            panic!("Send Frame not parsed correctly");
        }
    }

    #[test]
    fn stomp_connect_frame_with_host() {
        let message = b"CONNECT\n\
                accept-version:1.2\n\
                host:stomp.github.org\n\
                heart-beat:0,5000\n\
                \n\
                \x00"
            .to_vec();

        if let Ok(ClientFrame::Connect(frame)) = ClientFrame::try_from(message) {
            println!("{:?}",frame);
        } else {
            panic!("Send Frame not parsed correctly");
        }
    }
}
