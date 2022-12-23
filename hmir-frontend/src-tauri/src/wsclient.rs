

use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};
use tokio::runtime::Builder;

pub struct RequestClient {
    pub client  : Client,
    pub runtime : tokio::runtime::Runtime
}

impl RequestClient {
    pub fn new(uri : std::string::String) -> Self {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let client = runtime.block_on(async {
            let uri: Uri = format!("ws://{}", uri).parse().unwrap();
            let (tx, rx) = WsTransportClientBuilder::default().build(uri).await.unwrap();
            let client: Client = ClientBuilder::default().build_with_tokio(tx, rx);
            // let response: String = client.request("ttyd-start", None).await.unwrap();
            client
        });

        RequestClient {
            client: client,
            runtime: runtime
        }
    }

    pub fn ttyd_start(&self) {
        self.runtime.block_on(async {
            let response: String = self.client.request("ttyd-start", None).await.unwrap();
            println!("{}",response);
        });
    }

    pub fn ttyd_stop(&self) {
        self.runtime.block_on(async {
            let response: String = self.client.request("ttyd-stop", None).await.unwrap();
            println!("{}",response);
        });
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
    use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
    use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};
    use anyhow;
    use futures::executor::block_on;

    #[test]
    fn ttyd_start_workd() {
        let client = RequestClient::new("172.30.24.123:5898".to_string());
        client.ttyd_start();
    }

    #[test]
    fn ttyd_stop_worked() {
        let client = RequestClient::new("172.30.24.123:5898".to_string());
        client.ttyd_start();
    }
}