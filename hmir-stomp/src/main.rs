mod stomp;
mod option;
mod server;
mod error;

use option::ServerOption;
use server::StompServer;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let _ = start_stomp_serve().await?;

    let option = ServerOption::parse_option()?;
    let mut server = StompServer::new(option);
    server.start().await?;

    Ok(())
}








