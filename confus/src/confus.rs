use crate::config::Config;
use crate::vpnaas;
use crate::vpnaas::proto::{Pubkey, User, UserConfig};
use base64::{engine::general_purpose::STANDARD_NO_PAD as base64, Engine as _};
use std::convert::TryFrom;
use tonic::{Request, Response, Status};

pub struct ConfusServer {
    config: Config,
    server_peer: vpnaas::proto::user_config::ServerPeer,
}

impl ConfusServer {
    pub fn from_config(config: Config) -> ConfusServer {
        let server_peer = vpnaas::proto::user_config::ServerPeer {
            endpoint: config.config_endpoint.clone(),
            pubkey: Some(Pubkey {
                bytes: <[u8; 32]>::try_from(
                    base64
                        .decode(config.config_pubkey.clone())
                        .expect("WG_SERVER_PUBKEY decode failed"),
                )
                .expect("Invalid private key length")
                .to_vec(),
            }),
            allowed_ips: config.config_allowed_ips.clone(),
        };

        ConfusServer {
            config,
            server_peer,
        }
    }

    async fn keys_client(&self) -> Result<vpnaas::KeysClient<tonic::transport::Channel>, Status> {
        let channel = tonic::transport::Channel::from_shared(self.config.gprc_keys_url.to_string())
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
            user_peer: Some(user_peer),
            server_peer: Some(self.server_peer.clone()),
            dns: self.config.config_dns.clone(),
        };

        log::info!("Sent config for user '{}'", username);

        Ok(Response::new(config))
    }
}
