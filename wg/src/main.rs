use std::env;

use base64::{engine::general_purpose::STANDARD_NO_PAD as base64, Engine as _};
use tonic::transport::Server;

mod server;
mod vpnaas;
mod wg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let privkey = base64
        .decode(env::var("WG_SERVER_PRIVKEY").expect("WG_SERVER_PRIVKEY variable is unset"))?
        .try_into()
        .expect("Invalid private key length");

    let addr = "0.0.0.0:4242".parse()?;
    let wg_server = server::WgServer::new(privkey).await;

    Server::builder()
        .add_service(vpnaas::WgServer::new(wg_server))
        .serve(addr)
        .await?;

    Ok(())
}
