use crate::vpnaas;
use crate::vpnaas::proto::{Empty, Peer, Peers, Success, User, UserPubkey};

use redis::Commands;
use std::net::Ipv4Addr;
use tonic::{Request, Response, Status};

const GETSET_IP_SCRIPT: &str = r#"
local username = KEYS[1]

local allowed_ip = redis.call("HGET", "username:to:allowed_ip", username)

if not allowed_ip then
    allowed_ip = redis.call("SPOP", "allowed_ips")

    if allowed_ip then
        redis.call("HSET", "username:to:allowed_ip", username, allowed_ip)
    end
end

return allowed_ip
"#;

pub struct Keys {
    redis_connection_pool: r2d2::Pool<redis::Client>,
}

impl Keys {
    pub fn new() -> Keys {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let pool = r2d2::Pool::builder().max_size(15).build(client).unwrap();

        Keys {
            redis_connection_pool: pool,
        }
    }

    fn redis(&self) -> Result<r2d2::PooledConnection<redis::Client>, Status> {
        self.redis_connection_pool
            .get()
            .map_err(|e| Status::from_error(e.into()))
    }

    fn getset_ip(&self, username: String) -> Result<u32, Status> {
        let mut c = self.redis()?;

        redis::Script::new(&GETSET_IP_SCRIPT)
            .key(&username)
            .invoke::<Option<u32>>(&mut c)
            .map_err(|e| Status::from_error(e.into()))?
            .ok_or(Status::resource_exhausted("No IPs left in pool"))
    }
}

#[tonic::async_trait]
impl vpnaas::proto::keys_server::Keys for Keys {
    async fn set_pubkey(&self, request: Request<UserPubkey>) -> Result<Response<Success>, Status> {
        let (username, pubkey): (String, [u8; 32]) = request.into_inner().try_into()?;
        let allowed_ip: u32 = self.getset_ip(username)?;

        self.redis()?
            .hset("allowed_ip:to:pubkey", &allowed_ip, &pubkey)
            .map_err(|e| Status::from_error(e.into()))?;

        Ok(Response::new(Success::default()))
    }

    async fn get_peer(&self, request: Request<User>) -> Result<Response<Peer>, Status> {
        let mut redis = self.redis()?;
        let username: String = request.into_inner().try_into()?;

        let allowed_ip: u32 = redis
            .hget("username:to:allowed_ip", &username)
            .map_err(|_| Status::not_found("User has no allocated IP"))?;

        let pubkey: Vec<u8> = redis
            .hget("allowed_ip:to:pubkey", &allowed_ip)
            .map_err(|_| Status::not_found("Allowed IP has no associated public key"))?;

        if pubkey.len() != 32 {
            return Err(Status::invalid_argument("Public key length is incorrect"));
        }

        Ok(Response::new(Peer { pubkey, allowed_ip }))
    }

    async fn get_all_peers(&self, request: Request<Empty>) -> Result<Response<Peers>, Status> {
        let peers: Vec<Peer> = self
            .redis()?
            .hgetall::<_, Vec<(u32, Vec<u8>)>>("allowed_ip:to:pubkey")
            .map_err(|e| Status::from_error(e.into()))?
            .into_iter()
            .filter(|(allowed_ip, pubkey)| {
                if *allowed_ip == 0 {
                    println!("Got undefined ip. Skipping...");
                    false
                } else if pubkey.len() != 32 {
                    println!(
                        "Got bad pubkey for ip {}. Skipping...",
                        Ipv4Addr::from(*allowed_ip)
                    );
                    false
                } else {
                    true
                }
            })
            .map(|(allowed_ip, pubkey)| Peer { allowed_ip, pubkey })
            .collect();

        Ok(Response::new(Peers { peers }))
    }
}
