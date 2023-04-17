pub mod proto {
    tonic::include_proto!("vpnaas");
}

pub use proto::keys_client::KeysClient;
pub use proto::wg_client::WgClient;
pub use proto::wg_server::WgServer;

use proto::{Peer, Pubkey};

use std::net::Ipv4Addr;
use tonic::Status;
use wireguard_uapi::set::{AllowedIp as WgAllowedIp, Peer as WgPeer};

impl TryFrom<Pubkey> for [u8; 32] {
    type Error = Status;

    fn try_from(pubkey: Pubkey) -> Result<Self, Self::Error> {
        let pubkey = pubkey
            .bytes
            .try_into()
            .map_err(|_| Status::invalid_argument("Invalid public key length"))?;

        Ok(pubkey)
    }
}

impl TryFrom<Peer> for WgPeer {
    type Error = Status;

    fn try_from(peer: Peer) -> Result<Self, Self::Error> {
        if peer.ip == 0 {
            return Err(Status::invalid_argument("IP is not specified"));
        }

        let ip = WgAllowedIp::from_ipaddr(&Ipv4Addr::from(peer.ip).into());

        let pubkey = peer
            .pubkey
            .ok_or(Status::invalid_argument("Public key not specified"))
            .and_then(|p| {
                p.bytes
                    .try_into()
                    .map_err(|_| Status::invalid_argument("Invalid public key length"))
            })?;

        Ok(WgPeer {
            allowed_ips: vec![ip],
            public_key: pubkey,
            flags: vec![],
            preshared_key: None,
            endpoint: None,
            persistent_keepalive_interval: None,
            protocol_version: None,
        })
    }
}
