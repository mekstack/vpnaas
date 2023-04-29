use base64::{engine::general_purpose::STANDARD_NO_PAD as base64, Engine as _};
use std::env;

pub struct Config {
    pub grpc_port: String,
    pub grpc_keys_url: String,
    pub wgdevice_interface_name: String,
    pub wgdevice_interface_port: u16,
    pub wgdevice_privkey: [u8; 32],
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            grpc_port: get_env_var_or_default("VPNAAS_GRPC_PORT", "80"),
            grpc_keys_url: get_env_var_or_default("VPNAAS_GRPC_KEYS_URL", "http://keys:80"),
            wgdevice_interface_name: get_env_var_or_default("VPNAAS_WGDEVICE_NAME", "wg0"),
            wgdevice_interface_port: get_env_var_or_default("VPNAAS_WGDEVICE_PORT", "51820")
                .parse()
                .expect("Invalid port number"),
            wgdevice_privkey: base64
                .decode(get_env_var("VPNAAS_WGDEVICE_PRIVKEY"))
                .expect("Private key decode failed")
                .try_into()
                .expect("Invalid private key length"),
        }
    }
}

fn get_env_var(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| panic!("Environment variable {} is unset", var_name))
}

fn get_env_var_or_default(var_name: &str, default_value: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| default_value.to_string())
}
