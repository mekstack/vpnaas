use r2d2;
use redis::Commands;

use tonic::{transport::Server, Request, Response, Status};

use vpnaas::keys_server::KeysServer;
use vpnaas::{AllowedIp, Empty, Peer, Peers, Pubkey, Success, User, UserPubkey};

const GET_IP_FOR_USERNAME_SCRIPT: &str = r#"
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

pub mod vpnaas {
    tonic::include_proto!("vpnaas");
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let keys = Keys::new();

    Server::builder()
        .add_service(KeysServer::new(keys))
        .serve(addr)
        .await
        .unwrap();
}

pub struct Keys {
    redis_connection_pool: r2d2::Pool<redis::Client>,
}

impl Keys {
    fn new() -> Keys {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let pool = r2d2::Pool::builder().max_size(15).build(client).unwrap();

        Keys {
            redis_connection_pool: pool,
        }
    }
}

impl From<Pubkey> for [u8; 32] {
    fn from(pubkey: Pubkey) -> Self {
        pubkey.bytes.try_into().unwrap()
    }
}

#[tonic::async_trait]
impl vpnaas::keys_server::Keys for Keys {
    async fn set_pubkey(&self, request: Request<UserPubkey>) -> Result<Response<Success>, Status> {
        let UserPubkey { user, pubkey } = request.into_inner();
        let pubkey: [u8; 32] = pubkey.unwrap().into();

        let User { username } = user.unwrap();
        // TODO check that string is not empty string

        let mut c = self.redis_connection_pool.get().unwrap();

        let allowed_ip: String = redis::Script::new(&GET_IP_FOR_USERNAME_SCRIPT)
            .key(&username)
            .invoke(&mut c)
            .unwrap();

        println!("Got ip for {}: {}", &username, &allowed_ip);

        let result: i32 = c
            .hset("allowed_ip:to:pubkey", &allowed_ip, &pubkey)
            .unwrap();

        Ok(Response::new(Success::default()))
    }

    async fn get_allowed_ip(&self, request: Request<User>) -> Result<Response<AllowedIp>, Status> {
        let User { username } = request.into_inner();

        let mut c = self.redis_connection_pool.get().unwrap();

        let allowed_ip: u32 = redis::Script::new(&GET_IP_FOR_USERNAME_SCRIPT)
            .key(&username)
            .invoke(&mut c)
            .unwrap();

        Ok(Response::new(AllowedIp { allowed_ip }))
    }

    async fn get_peer(&self, request: Request<User>) -> Result<Response<Peer>, Status> {
        let mut c = self.redis_connection_pool.get().unwrap();

        let User { username } = request.into_inner();

        // lookup allowed_ip
        let allowed_ip: u32 = c.hget("username:to:allowed_ip", &username).unwrap();
        let pubkey: Vec<u8> = c.hget("allowed_ip:to:pubkey", &allowed_ip).unwrap();

        Ok(Response::new(Peer { pubkey, allowed_ip }))
    }

    async fn get_all_peers(&self, request: Request<Empty>) -> Result<Response<Peers>, Status> {
        let mut c = self.redis_connection_pool.get().unwrap();

        let peers: Vec<(u32, Vec<u8>)> = c.hgetall("allowed_ip:to:pubkey").unwrap();

        let peers: Vec<Peer> = peers
            .into_iter()
            .map(|(allowed_ip, pubkey)| Peer { allowed_ip, pubkey })
            .collect();

        Ok(Response::new(Peers { peers }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;
    use tonic::transport::Channel;

    async fn create_test_server() {
        let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

        tokio::spawn(async move {
            Server::builder()
                .add_service(KeysServer::new(Keys::new()))
                .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)) // full schizo
                .await
                .unwrap();
        });

        // Populate Redis with sample data
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut c = client.get_connection().unwrap();

        redis::cmd("FLUSHDB").query::<()>(&mut c).unwrap();

        let _: i32 = c
            .sadd(
                "allowed_ips",
                vec![u32::from(std::net::Ipv4Addr::new(192, 168, 1, 1))],
            )
            .unwrap();
    }

    async fn create_test_client() -> vpnaas::keys_client::KeysClient<Channel> {
        let addr = "http://127.0.0.1:3000";
        let channel = tonic::transport::Channel::from_static(addr)
            .connect()
            .await
            .unwrap();

        let client = vpnaas::keys_client::KeysClient::new(channel);
        client
    }

    #[tokio::test]
    async fn test_keys() {
        create_test_server().await;

        let pubkey_b64 = "baTHPpnl0BBtC5f0kcn3maRCD2dHpyv1eK1gLVXp1i0=";
        let mut client = create_test_client().await;

        let user = User {
            username: String::from("test"),
        };

        let user_pubkey: UserPubkey = UserPubkey {
            user: Some(user.clone()),
            pubkey: Some(Pubkey {
                bytes: base64::decode(pubkey_b64).unwrap(),
            }),
        };

        client.set_pubkey(user_pubkey).await.unwrap();

        let peer = client.get_peer(user).await.unwrap().into_inner();

        assert!(
            peer == Peer {
                allowed_ip: u32::from(std::net::Ipv4Addr::new(192, 168, 1, 1)),
                pubkey: base64::decode(pubkey_b64).unwrap()
            }
        )
    }

    #[tokio::test]
    async fn test_get_allowed_ip_fail() {
        // Test failure when user was not added
        todo!()
    }

    #[tokio::test]
    async fn test_get_all_peers() {
        todo!()
    }
}
