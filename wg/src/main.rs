mod config;
mod server;
mod vpnaas;
mod wg;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = config::Config::from_env();
    let grpc_server_url = format!("0.0.0.0:{}", config.grpc_port).parse()?;
    let wg_server = server::WgServer::new(config).await;

    log::info!("Starting wg gRPC server on {}", grpc_server_url);

    Server::builder()
        .add_service(vpnaas::WgServer::new(wg_server))
        .serve(grpc_server_url)
        .await?;

    Ok(())
}
