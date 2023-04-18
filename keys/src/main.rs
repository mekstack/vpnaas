mod keys;
mod tests;
mod vpnaas;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:3000".parse()?;
    let keys_server = keys::KeysServer::new();

    Server::builder()
        .add_service(vpnaas::KeysServer::new(keys_server))
        .serve(addr)
        .await?;

    Ok(())
}
