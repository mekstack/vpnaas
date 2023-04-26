mod config;
mod jwt;
mod keys;
mod tests;
mod vpnaas;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = config::Config::from_env();
    let jwt = jwt::JwtValidator::new(config.jwt_secret_key.clone());

    let server_url = format!("0.0.0.0:{}", config.server_port).parse()?;
    let keys_server = vpnaas::KeysServer::new(keys::KeysServer::new(config, jwt));

    log::info!("Starting keys server on {}", server_url);

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(keys_server))
        .serve(server_url)
        .await?;

    Ok(())
}
