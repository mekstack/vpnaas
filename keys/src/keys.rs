use crate::vpnaas;
use crate::vpnaas::proto::{Empty, Peer, Peers, Pubkey, Success, User, UserPubkey};

use crate::jwt;
use redis::Commands;
use std::net::Ipv4Addr;
use std::time::Duration;
use tonic::{Request, Response, Status};

const GETSET_IP_SCRIPT: &str = r#"
local username = ARGV[1]

local ip = redis.call("HGET", "username:to:ip", username)

if not ip then
    ip = redis.call("SPOP", "ip_pool")

    if ip then
        redis.call("HSET", "username:to:ip", username, ip)
    end
end

return ip
"#;

const SET_PUBKEY_NX_SCRIPT: &str = r#"
local pubkey = ARGV[1]
local username = ARGV[2]
local ip = ARGV[3]

local existing_username = redis.call("HGET", "pubkey:to:username", username)

if not existing_username then
    redis.call("HSET", "ip:to:pubkey", ip, pubkey)
    redis.call("HSET", "pubkey:to:username", pubkey, username)
end

return existing_username
"#;

pub struct KeysServer {
    redis_connection_pool: r2d2::Pool<redis::Client>,
    jwt: jwt::JwtValidator,
}

impl KeysServer {
    pub fn new(jwt: jwt::JwtValidator) -> KeysServer {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let pool = r2d2::Pool::builder()
            .connection_timeout(Duration::from_secs(1))
            .max_size(15)
            .build(client)
            .unwrap();

        KeysServer {
            redis_connection_pool: pool,
            jwt,
        }
    }

    fn redis(&self) -> Result<r2d2::PooledConnection<redis::Client>, Status> {
        self.redis_connection_pool
            .get()
            .map_err(|e| Status::from_error(e.into()))
    }

    fn getset_ip(&self, username: &String) -> Result<u32, Status> {
        let mut c = self.redis()?;

        redis::Script::new(&GETSET_IP_SCRIPT)
            .arg(username)
            .invoke::<Option<u32>>(&mut c)
            .map_err(|e| Status::from_error(e.into()))?
            .ok_or(Status::resource_exhausted("No IPs left in pool"))
    }

    fn set_pubkey_nx(&self, pubkey: &[u8; 32], username: &String, ip: u32) -> Result<(), Status> {
        let mut c = self.redis()?;

        let existing_username = redis::Script::new(&SET_PUBKEY_NX_SCRIPT)
            .arg(pubkey)
            .arg(username)
            .arg(ip)
            .invoke::<Option<String>>(&mut c)
            .map_err(|e| Status::from_error(e.into()))?;

        if let Some(existing_username) = existing_username {
            if username != &existing_username {
                return Err(Status::already_exists("Public key is already in use"));
            }
        }
        Ok(())
    }

    async fn wg_server_client(
        &self,
    ) -> Result<vpnaas::WgClient<tonic::transport::Channel>, Status> {
        let channel = tonic::transport::Channel::from_static("http://127.0.0.1:4242")
            .connect()
            .await
            .map_err(|_| Status::unavailable("Connection to Wireguard Server failed"))?;

        Ok(vpnaas::WgClient::new(channel))
    }
}

#[tonic::async_trait]
impl vpnaas::proto::keys_server::Keys for KeysServer {
    async fn set_pubkey(&self, request: Request<UserPubkey>) -> Result<Response<Success>, Status> {
        let (metadata, _, inner) = request.into_parts();
        let (username, pubkey): (String, [u8; 32]) = inner.try_into()?;
        self.jwt.validate(&username, &metadata)?;

        let ip: u32 = self.getset_ip(&username)?;
        self.set_pubkey_nx(&pubkey, &username, ip)?;

        self.wg_server_client()
            .await?
            .push_peer_update(Peer {
                ip,
                pubkey: Some(Pubkey {
                    bytes: pubkey.into(),
                }),
            })
            .await?;

        Ok(Response::new(Success::default()))
    }

    async fn get_peer(&self, request: Request<User>) -> Result<Response<Peer>, Status> {
        let mut redis = self.redis()?;
        let username: String = request.into_inner().try_into()?;

        let ip: u32 = redis
            .hget::<_, _, Option<_>>("username:to:ip", &username)
            .map_err(|e| Status::from_error(e.into()))?
            .ok_or(Status::not_found("User has no allocated IP"))?;

        let pubkey: Vec<u8> = redis
            .hget::<_, _, Option<_>>("ip:to:pubkey", &ip)
            .map_err(|e| Status::from_error(e.into()))?
            .ok_or(Status::not_found("IP has no associated public key"))?;

        if pubkey.len() != 32 {
            return Err(Status::invalid_argument("Public key length is incorrect"));
        }

        Ok(Response::new(Peer {
            ip,
            pubkey: Some(Pubkey { bytes: pubkey }),
        }))
    }

    async fn get_all_peers(&self, request: Request<Empty>) -> Result<Response<Peers>, Status> {
        let peers: Vec<Peer> = self
            .redis()?
            .hgetall::<_, Vec<(u32, Vec<u8>)>>("ip:to:pubkey")
            .map_err(|e| Status::from_error(e.into()))?
            .into_iter()
            .filter(|(ip, pubkey)| {
                if *ip == 0 {
                    log::warn!("Got undefined ip. Skipping...");
                    false
                } else if pubkey.len() != 32 {
                    log::warn!("Got bad pubkey for ip {}. Skipping...", Ipv4Addr::from(*ip));
                    false
                } else {
                    true
                }
            })
            .map(|(ip, pubkey)| Peer {
                ip,
                pubkey: Some(Pubkey { bytes: pubkey }),
            })
            .collect();

        Ok(Response::new(Peers { peers }))
    }
}
