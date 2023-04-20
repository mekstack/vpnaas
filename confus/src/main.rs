mod confus;
mod vpnaas;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let server_url = "0.0.0.0:4448".parse()?;
    let config_gen_server = confus::ConfigGenServer::from_env();

    log::info!("Starting confus server on {}", server_url);

    Server::builder()
        .add_service(vpnaas::ConfusServer::new(config_gen_server))
        .serve(server_url)
        .await?;

    Ok(())
}
