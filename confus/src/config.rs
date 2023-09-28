use std::env;

pub struct Config {
    pub grpc_port: String,
    pub grpc_keys_url: String,
    pub interface_config: Vec<String>,
    pub peer_config: Vec<String>,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            grpc_port: get_env_var_or_default("VPNAAS_GRPC_PORT", "80"),
            grpc_keys_url: get_env_var_or_default("VPNAAS_GRPC_KEYS_URL", "http://keys:80"),
            interface_config: get_env_var("VPNAAS_INTERFACE_CONFIG")
                .lines()
                .map(|s| s.trim().to_string())
                .collect(),
            peer_config: get_env_var("VPNAAS_PEER_CONFIG")
                .lines()
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }
}

fn get_env_var(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| panic!("Environment variable {} is unset", var_name))
}

fn get_env_var_or_default(var_name: &str, default_value: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| default_value.to_string())
}
