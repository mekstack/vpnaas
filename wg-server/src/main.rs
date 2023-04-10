use anyhow;
use base64::{engine::general_purpose, Engine as _};
use std::env;

use wireguard_uapi::get::{AllowedIp, Device, Peer as DevicePeer};
use wireguard_uapi::set;
use wireguard_uapi::DeviceInterface;

use tonic::{transport::Server, Request, Response, Status};

use vpnaas::wg_server::{Wg, WgServer};
use vpnaas::{Peer, PeerPushStatus};

pub mod vpnaas {
    tonic::include_proto!("vpnaas");
}

#[derive(Debug, Default)]
pub struct MyWg {}

#[tonic::async_trait]
impl Wg for MyWg {
    async fn push_new_peer(
        &self,
        request: Request<Peer>,
    ) -> Result<Response<PeerPushStatus>, Status> {
        let mut device = set::Device::from_ifname("wg0".to_string());

        let private_key: [u8; 32] = general_purpose::STANDARD_NO_PAD
            .decode("sFS4WEV2m+bAe2O+qOxMYmz78VB+aQGECxAMXNeW92w")
            .unwrap()
            .try_into()
            .unwrap();
        device.private_key = Some(&private_key);

        let public_key: [u8; 32] = request.into_inner().pubkey.try_into().unwrap();
        let peer = wireguard_uapi::linux::set::Peer::from_public_key(&public_key);
        device.peers.push(peer);

        let mut wg = wireguard_uapi::WgSocket::connect().unwrap();
        let resp;
        match wg.set_device(device) {
            Ok(()) => {
                resp = PeerPushStatus { added: true };
            }
            Err(_) => {
                resp = PeerPushStatus { added: false };
            }
        };

        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let wg_server = MyWg::default();

    Server::builder()
        .add_service(WgServer::new(wg_server))
        .serve(addr)
        .await;
}
