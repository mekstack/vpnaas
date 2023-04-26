use futures::StreamExt;
use netlink_packet_core::{NetlinkMessage, NetlinkPayload, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_generic::GenlMessage;
use netlink_packet_wireguard::constants::AF_INET;
use netlink_packet_wireguard::nlas;
use netlink_packet_wireguard::nlas::{WgAllowedIp, WgAllowedIpAttrs, WgDeviceAttrs, WgPeerAttrs};
use netlink_packet_wireguard::{Wireguard, WireguardCmd};
use std::net::IpAddr;

type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct WgDevice {
    ifname: String,
    private_key: [u8; 32],
    listen_port: u16,
    peers: Vec<WgPeer>,
}

#[derive(Debug)]
pub struct WgPeer {
    pub public_key: [u8; 32],
    pub allowed_ip: IpAddr,
}

impl WgDevice {
    pub async fn new(ifname: &str, listen_port: u16, private_key: [u8; 32]) -> Self {
        set_device(vec![
            WgDeviceAttrs::IfName(ifname.to_string()),
            WgDeviceAttrs::PrivateKey(private_key.clone()),
            WgDeviceAttrs::ListenPort(listen_port),
        ])
        .await
        .unwrap();

        WgDevice {
            ifname: ifname.to_string(),
            private_key,
            listen_port,
            peers: vec![],
        }
    }

    pub async fn update_peer(&mut self, new_peer: WgPeer) -> Result<(), BoxedError> {
        if let Some(peer) = self
            .peers
            .iter_mut()
            .find(|peer| peer.allowed_ip == new_peer.allowed_ip)
        {
            *peer = new_peer;
        } else {
            self.peers.push(new_peer);
        }

        self.set_peers().await
    }

    pub async fn extend_peers(&mut self, new_peers: Vec<WgPeer>) -> Result<(), BoxedError> {
        if new_peers.len() == 0 {
            return Ok(());
        }
        self.peers = new_peers;
        self.set_peers().await
    }

    async fn set_peers(&self) -> Result<(), BoxedError> {
        let device_attrs = vec![
            WgDeviceAttrs::IfName(self.ifname.clone()),
            WgDeviceAttrs::Peers(
                self.peers
                    .iter()
                    .map(|short_peer| nlas::WgPeer {
                        0: vec![
                            WgPeerAttrs::PublicKey(short_peer.public_key.clone()),
                            WgPeerAttrs::AllowedIps {
                                0: vec![WgAllowedIp {
                                    0: vec![
                                        WgAllowedIpAttrs::IpAddr(short_peer.allowed_ip),
                                        WgAllowedIpAttrs::Cidr(32),
                                        WgAllowedIpAttrs::Family(AF_INET),
                                    ],
                                }],
                            },
                        ],
                    })
                    .collect(),
            ),
        ];

        set_device(device_attrs).await
    }
}

async fn set_device(device_attributes: Vec<WgDeviceAttrs>) -> Result<(), BoxedError> {
    let (connection, mut handle, _) = genetlink::new_connection()?;
    tokio::spawn(connection);

    let genlmsg: GenlMessage<Wireguard> = GenlMessage::from_payload(Wireguard {
        cmd: WireguardCmd::SetDevice,
        nlas: device_attributes,
    });

    let mut nlmsg = NetlinkMessage::from(genlmsg);
    nlmsg.header.flags = NLM_F_REQUEST | NLM_F_ACK;

    let mut res = handle.request(nlmsg).await?;
    while let Some(rx_packet) = res.next().await {
        match rx_packet?.payload {
            NetlinkPayload::Error(e) => return Err(e.to_string().into()),
            _ => (),
        };
    }

    Ok(())
}
