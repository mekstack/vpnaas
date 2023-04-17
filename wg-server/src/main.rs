use std::env;

use base64::{engine::general_purpose::STANDARD_NO_PAD as base64, Engine as _};
use tonic::transport::Server;

use vpnaas::WgServer;

mod vpnaas;
mod wg_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let privkey = base64
        .decode(env::var("WG_SERVER_PRIVKEY").unwrap())?
        .try_into()
        .expect("Invalid private key length");

    let addr = "0.0.0.0:4242".parse()?;
    let wg_server = wg_server::Wg::new(privkey).await;

    Server::builder()
        .add_service(WgServer::new(wg_server))
        .serve(addr)
        .await?;

    Ok(())
}
