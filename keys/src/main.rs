mod jwt;
mod keys;
mod tests;
mod vpnaas;

use std::env;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:3000".parse()?;

    let jwt_secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY variable is unset");
    let jwt = jwt::JwtValidator::new(jwt_secret_key);
    let keys_server = vpnaas::KeysServer::new(keys::KeysServer::new(jwt));

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(keys_server))
        .serve(addr)
        .await?;

    Ok(())
}
