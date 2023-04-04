use anyhow;
use base64::{engine::general_purpose, Engine as _};
use std::env;

use wireguard_uapi::set;
use wireguard_uapi::get::{AllowedIp, Device, Peer};
use wireguard_uapi::DeviceInterface;

// TODO(mmskv): openstackesque logging
//use log::{info, warn};

fn main() -> anyhow::Result<()> {
    let device_name = match env::var("WG_INTERFACE") {
        Ok(result) => result,
        Err(_) => {
            println!("WG_INTERFACE not set. Using default 'wg0'");
            "wg0".to_string()
        }
    };

    let mut wg = wireguard_uapi::WgSocket::connect()?;
    // let mut device = wg
    //     .get_device(DeviceInterface::from_name(device_name))
    //     .unwrap();
    let mut device = set::Device::from_ifname(device_name);

    let private_key: [u8; 32] = general_purpose::STANDARD_NO_PAD
        .decode("sFS4WEV2m+bAe2O+qOxMYmz78VB+aQGECxAMXNeW92w")
        .unwrap()
        .try_into()
        .unwrap();

    let public_key: [u8; 32] = general_purpose::STANDARD_NO_PAD
        .decode("baTHPpnl0BBtC5f0kcn3maRCD2dHpyv1eK1GLVXp1i0")
        .unwrap()
        .try_into()
        .unwrap();

    let peer = wireguard_uapi::linux::set::Peer::from_public_key(&public_key);

    device.private_key = Some(&private_key);
    device.peers.push(peer);

    wg.set_device(device);

    // println!("{:?}", device);

    while true {}

    Ok(())
}
