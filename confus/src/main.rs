mod config;
mod confus;
mod vpnaas;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = config::Config::from_env();
    let server_url = format!("0.0.0.0:{}", config.server_port).parse()?;
    let confus_server = vpnaas::ConfusServer::new(confus::ConfusServer::from_config(config));

    log::info!("Starting confus server on {}", server_url);

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(confus_server))
        .serve(server_url)
        .await?;

    Ok(())
}
