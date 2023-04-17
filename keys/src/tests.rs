#[cfg(test)]
mod tests {
    use crate::keys::Keys;
    use crate::vpnaas;
    use crate::vpnaas::proto::{Peer, Pubkey, User, UserPubkey};

    use redis::Commands;
    use std::net::Ipv4Addr;
    use tokio::net::TcpListener;
    use tonic::transport::{Channel, Server};

    async fn create_test_server() {
        let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

        tokio::spawn(async move {
            Server::builder()
                .add_service(vpnaas::KeysServer::new(Keys::new()))
                .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
                .await
                .unwrap();
        });

        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut c = client.get_connection().unwrap();

        redis::cmd("FLUSHDB").query::<()>(&mut c).unwrap();

        let _: i32 = c
            .sadd(
                "ip_pool",
                vec![u32::from(Ipv4Addr::new(192, 168, 1, 1))],
            )
            .unwrap();
    }

    async fn create_test_client() -> vpnaas::KeysClient<Channel> {
        let addr = "http://127.0.0.1:3000";
        let channel = tonic::transport::Channel::from_static(addr)
            .connect()
            .await
            .unwrap();

        let client = vpnaas::KeysClient::new(channel);
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
                allowed_ip: u32::from(Ipv4Addr::new(192, 168, 1, 1)),
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
