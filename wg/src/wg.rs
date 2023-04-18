use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use wireguard_uapi::set::{Device as WgDevice, Peer as WgPeer};
use wireguard_uapi::{DeviceInterface as WgDeviceInterface, WgSocket};

use crate::vpnaas;
use crate::vpnaas::proto::{Empty, Success};

pub struct WgServer {
    device: Mutex<WgDevice>,
    socket: Mutex<WgSocket>,
}

async fn keys_client() -> vpnaas::KeysClient<tonic::transport::Channel> {
    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:3000")
        .connect()
        .await
        .expect("Connection to Keys server failed");
    vpnaas::KeysClient::new(channel)
}

impl WgServer {
    pub async fn new(private_key: [u8; 32]) -> WgServer {
        let mut client = keys_client().await;
        let mut socket = WgSocket::connect().expect("Could not connect to wg socket");

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

        let device = WgDevice {
            interface: WgDeviceInterface::Name("wg0".to_string()),
            private_key,
            listen_port: 6969,
            fwmark: None,
            flags: vec![],
            peers,
        };

        socket
            .set_device(&device)
            .expect("Wireguard device setup failed");

        WgServer {
            device: Mutex::new(device),
            socket: Mutex::new(socket),
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

        if let Some(peer) = device
            .peers
            .iter_mut()
            .find(|peer| peer.allowed_ips == new_peer.allowed_ips)
        {
            *peer = new_peer;
        } else {
            device.peers.push(new_peer);
        }

        self.socket
            .lock()
            .await
            .set_device(&device)
            .map_err(|e| Status::from_error(e.into()))?;

        Ok(Response::new(Success::default()))
    }
}
