mod stomp;
mod tls;

use tokio::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, split};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::futures;
use tokio_rustls::server::TlsStream;
use crate::stomp::handle_frame;
use parser::client::ClientFrame;
use tls::TlsOption;

const BUFFER_SIZE: usize = 4096;
const SERVER_CERT_FILE: &str = "/etc/pki/vdsm/certs/vdsmcert.pem";
const SERVER_KEY_FILE: &str = "/etc/pki/vdsm/keys/vdsmkey.pem";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = start_stomp_serve().await?;
    Ok(())
}

async fn stomp_server(listener: TcpListener) -> anyhow::Result<()>
{
    loop { // 接受传入的客户端TCP 连接
        let (stream, _addr) = listener.accept().await?;
        tokio::task::spawn(async move {
            let _ = handle_connection(stream).await;
        });
    }
}


async fn start_stomp_serve() -> anyhow::Result<()>
{

    let mut tls_option = TlsOption {
        cert : SERVER_CERT_FILE.to_string(),
        key: SERVER_KEY_FILE.to_string()
    };

    let accept = tls_option.get_tls_accept().await.ok();
    let listener = TcpListener::bind("172.30.21.13:54321").await?;


    loop {
        tokio::select! {
            v = listener.accept() => {
                let (stream, _) = v?;
                if let Some(a) = accept.clone() {
                    let stream = a.accept(stream).await;
                    // 获取的流跟正常内容一样读写, 在内部实现了自动加解密
                    if let Ok(stream) = stream {
                        // let _ = self.deal_stream(inbound, client.clone()).await;
                        tokio::task::spawn(async move {
                            let _ = handle_tls_connection(stream).await;
                        });
                    } else {
                        println!("accept error = {:?}", stream.err());
                    }
                } else {
                    // let _ = self.deal_stream(inbound, client.clone()).await;
                };
            }
        }
        println!("aaaaaaaaaaaaaa");
    }


    Ok(())
}

fn stomp_debug(frame : & Vec<u8>)
{
    println!("{}",String::from_utf8_lossy(frame));
}

//一个tls客户端连接
async fn handle_tls_connection<S>(mut stream: S) -> anyhow::Result<(),Box<dyn std::error::Error>>
    where
        S: AsyncRead + AsyncWrite + Unpin + AsyncWriteExt,
{

    let (mut local_reader, mut local_writer) = split(stream);

    loop {
        let mut buffer = [0u8; BUFFER_SIZE];
        let result = local_reader.read(&mut buffer).await;
        match result {
            Ok(0) => {//客户端关闭连接
                println!("Client closed!");
                break;
            } ,
            Ok(n) => {//读取到数据，对数据进行处理
                let frame_buffer = Vec::from(&buffer[0..n]);
                stomp_debug(&frame_buffer);
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

//一个客户端连接
async fn handle_connection(stream: TcpStream) -> anyhow::Result<(),Box<dyn std::error::Error>>  {

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
                handle_frame(frame_buffer,&stream).await;
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
