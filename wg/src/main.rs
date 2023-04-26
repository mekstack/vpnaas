mod config;
mod server;
mod vpnaas;
mod wg;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = config::Config::from_env();
    let server_url = format!("0.0.0.0:{}", config.server_port).parse()?;
    let wg_server = server::WgServer::new(config).await;

    log::info!("Starting wg server on {}", server_url);

    Server::builder()
        .add_service(vpnaas::WgServer::new(wg_server))
        .serve(server_url)
        .await?;

    Ok(())
}
