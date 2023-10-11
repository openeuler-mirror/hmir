use tokio::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, split};
use tokio::net::{TcpListener, TcpStream};
use crate::ServerOption;

const BUFFER_SIZE: usize = 4096;


pub struct StompServer {
    option : ServerOption,
}

impl StompServer {

    pub fn new(option: ServerOption) -> Self {
        Self {
            option : option
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()>{


        let address = self.option.ip.clone().unwrap() + ":" + self.option.port.clone().unwrap().as_str();
        let acceptor = self.option.get_tls_accept().await.ok();
        let listener = TcpListener::bind(address).await?;

        loop {
            tokio::select! {
                v = listener.accept() => {
                    let (stream, _) = v?;
                    if let Some(tls_acceptor) = acceptor.clone() {
                        let stream = tls_acceptor.accept(stream).await;
                        // 获取的流跟正常内容一样读写, 在内部实现了自动加解密
                        if let Ok(stream) = stream {
                            tokio::task::spawn(async move {
                                let _ = Self::handle_tls_connection(stream).await;
                            });
                        } else {
                            println!("accept error = {:?}", stream.err());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    //一个tls客户端连接
    async fn handle_tls_connection<S>(mut stream: S) -> anyhow::Result<(),Box<dyn std::error::Error>>
        where
            S: AsyncRead + AsyncWrite + Unpin + Send + Sync + 'static,
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
                    Self::debug(&frame_buffer);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }


        Ok(())
    }

    fn debug(frame : & Vec<u8>) {
        println!("{}",String::from_utf8_lossy(frame));
    }

    //一个客户端连接
    async fn handle_connection(stream: TcpStream) -> anyhow::Result<()>  {

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
                    // handle_frame(frame_buffer,&stream).await;
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

}