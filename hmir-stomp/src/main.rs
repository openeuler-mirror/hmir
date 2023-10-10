mod stomp;

use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::unbounded_channel as channel;

use std::convert::TryFrom;

use parser::client::ClientFrame;
use parser::headers::HeaderValue;

const BUFFER_SIZE: usize = 4096;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("172.30.21.13:61613").await?;
    stomp_server(listener).await;
    Ok(())
}

async fn stomp_server(listener: TcpListener) -> anyhow::Result<()>
{
    loop { // 接受传入的客户端TCP 连接
        let (stream, _addr) = listener.accept().await?;
        tokio::task::spawn(async move {
            handle_connection(stream).await;
        });
    }
}


fn stomp_debug(frame : & Vec<u8>)
{
    println!("{}",String::from_utf8_lossy(frame));
}


//一个客户端连接
async fn handle_connection(mut stream: TcpStream) -> anyhow::Result<(),Box<dyn std::error::Error>>  {

    loop {
        // Wait for the socket to be readable
        stream.readable().await?;

        let mut buffer = vec![0u8; BUFFER_SIZE];

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut buffer) {
            Ok(0) => {//客户端关闭连接
                println!("Client closed!");
                break;
            } ,
            Ok(n) => {//读取到数据，对数据进行处理
                let frame_buffer = Vec::from(&buffer[0..n]);
                stomp_debug(&frame_buffer);
                let frame =  ClientFrame::try_from(frame_buffer);
                // if let Ok(ClientFrame::Send(f)) = frame {
                //
                // } else if let Ok(ClientFrame::Connect(f)) = frame {
                //     println!("{}",f);
                // } else {
                //     println!("---------");
                // }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                //
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
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
            // println!("{}",frame);
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
