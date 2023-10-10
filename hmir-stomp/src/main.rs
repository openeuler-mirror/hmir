use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::unbounded_channel as channel;

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


