use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
use jsonrpsee::core::client::{Client, ClientBuilder};
use tokio::runtime::Builder;

use log::{error};


#[derive(Debug)]
pub struct RequestClient {
    pub client  : Client,
    pub token   : String,
    pub runtime : tokio::runtime::Runtime,
}

macro_rules! client_check {
    ( $client : expr ) =>{
        if !$client.is_connected(){
            return (errno::HMIR_ERR_CONNECT_SERVER,"".to_string());
        }
    }
}


impl RequestClient {

    //根据uri返回一个wc client
    pub fn new(uri : String) -> Result<Self,bool> {


        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let client = runtime.block_on(async {
            let uri: Uri = format!("ws://{}", uri).parse().unwrap();
            let client_builder = WsTransportClientBuilder::default().build(uri.clone()).await;
            match client_builder {
                Ok((tx,rx)) => {
                    let client: Client = ClientBuilder::default().build_with_tokio(tx, rx);
                    Ok(client)
                },
                Err(_e) => {
                    error!("Connect the remote {} failed, Reason : {}",uri.clone(),_e.to_string());
                    Err(false)
                }
            }
        });

        match client {
            Ok(c) => {
                Ok(RequestClient{ client: c, token: "".to_string(), runtime: runtime })
            },
            Err(_e) => Err(false)
        }
    }
}




