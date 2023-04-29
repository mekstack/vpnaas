use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::vpnaas;
use crate::vpnaas::proto::{Empty, Success};

use crate::config::Config;
use crate::wg::{WgDevice, WgPeer};

pub struct WgServer {
    device: Mutex<WgDevice>,
}

async fn keys_client(url: String) -> vpnaas::KeysClient<tonic::transport::Channel> {
    let channel = tonic::transport::Channel::from_shared(url.to_string())
        .expect("Invalid keys service url")
        .connect()
        .await
        .expect("Connection to Keys server failed");
    vpnaas::KeysClient::new(channel)
}

impl WgServer {
    pub async fn new(config: Config) -> WgServer {
        let mut client = keys_client(config.grpc_keys_url).await;

        let peers = client
            .get_all_peers(Empty::default())
            .await
            .expect("Could not get peers from Keys server")
            .into_inner()
            .peers;

        let peers: Vec<WgPeer> = peers
            .into_iter()
            .filter_map(|p| p.try_into().ok())
            .collect();

        let mut device = WgDevice::new(
            &config.wgdevice_interface_name,
            config.wgdevice_interface_port,
            config.wgdevice_privkey,
        )
        .await;

        device
            .extend_peers(peers)
            .await
            .expect("Wireguard device setup failed");

        WgServer {
            device: Mutex::new(device),
        }
    }
}

#[tonic::async_trait]
impl vpnaas::proto::wg_server::Wg for WgServer {
    async fn push_peer_update(
        &self,
        request: Request<vpnaas::proto::Peer>,
    ) -> Result<Response<Success>, Status> {
        let new_peer: WgPeer = request.into_inner().try_into()?;

        let mut device = self.device.lock().await;

        device
            .update_peer(new_peer)
            .await
            .map_err(|e| Status::from_error(e))?;

        Ok(Response::new(Success::default()))
    }
}
