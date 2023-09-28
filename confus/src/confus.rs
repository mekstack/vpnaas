use crate::config::Config;
use crate::vpnaas;
use crate::vpnaas::proto::{User, UserConfig};
use tonic::{Request, Response, Status};

pub struct ConfusServer {
    config: Config,
}

impl ConfusServer {
    pub fn from_config(config: Config) -> ConfusServer {
        Self { config }
    }

    async fn keys_client(&self) -> Result<vpnaas::KeysClient<tonic::transport::Channel>, Status> {
        let channel = tonic::transport::Channel::from_shared(self.config.grpc_keys_url.to_string())
            .map_err(|_| Status::unavailable("Invalid keys service url"))?
            .connect()
            .await
            .map_err(|_| Status::unavailable("Connection to Keys Service failed"))?;

        Ok(vpnaas::KeysClient::new(channel))
    }
}

#[tonic::async_trait]
impl vpnaas::proto::confus_server::Confus for ConfusServer {
    async fn get_config(&self, request: Request<User>) -> Result<Response<UserConfig>, Status> {
        let User { username } = request.into_inner();

        let user_peer = self
            .keys_client()
            .await?
            .get_peer(vpnaas::proto::User {
                username: username.clone(),
            })
            .await?
            .into_inner();

        let config = UserConfig {
            user: Some(user_peer),
            interface_config: self.config.interface_config.clone(),
            peer_config: self.config.peer_config.clone(),
        };

        log::info!("Sent config for user '{}'", username);

        Ok(Response::new(config))
    }
}
