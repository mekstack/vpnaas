use base64::{engine::general_purpose::STANDARD_NO_PAD as base64, Engine as _};
use std::env;

pub struct Config {
    pub server_port: String,
    pub private_key: [u8; 32],
    pub keys_client_url: String,
    pub iface_name: String,
    pub iface_port: u16,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            server_port: get_env_var_or_default("WG_SERVER_PORT", "80"),
            keys_client_url: get_env_var("KEYS_CLIENT_URL"),
            iface_name: get_env_var("WG_IFACE_NAME"),
            iface_port: get_env_var("WG_IFACE_PORT")
                .parse()
                .expect("Invalid port number"),
            private_key: base64
                .decode(get_env_var("WG_SERVER_PRIVKEY"))
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
