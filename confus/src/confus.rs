use crate::vpnaas;
use crate::vpnaas::proto::{Pubkey, User, UserConfig};
use base64::{engine::general_purpose::STANDARD_NO_PAD as base64, Engine as _};
use std::env;
use tonic::{Request, Response, Status};

pub struct ConfusServer {
    keys_url: String,
    dns: String,
    server_peer: vpnaas::proto::user_config::ServerPeer,
}

fn get_env_var(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| panic!("Environment variable {} is unset", var_name))
}

impl ConfusServer {
    pub fn from_env() -> ConfusServer {
        let server_peer = vpnaas::proto::user_config::ServerPeer {
            endpoint: get_env_var("WG_SERVER_ENDPOINT"),
            pubkey: Some(Pubkey {
                bytes: <[u8; 32]>::try_from(
                    base64
                        .decode(get_env_var("WG_SERVER_PUBKEY"))
                        .expect("WG_SERVER_PUBKEY decode failed"),
                )
                .expect("Invalid private key length")
                .to_vec(),
            }),

            allowed_ips: get_env_var("ALLOWED_IPS")
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
        };

        ConfusServer {
            keys_url: get_env_var("KEYS_URL"),
            dns: get_env_var("DNS_CONFIG"),
            server_peer,
        }
    }

    async fn keys_client(&self) -> Result<vpnaas::KeysClient<tonic::transport::Channel>, Status> {
        let channel = tonic::transport::Channel::from_shared(self.keys_url.clone().into_bytes())
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
            dns: self.dns.clone(),
        };

        log::info!("Sent config for user '{}'", username);

        Ok(Response::new(config))
    }
}
