use std::env;

pub struct Config {
    pub grpc_port: String,
    pub config_endpoint: String,
    pub config_pubkey: String,
    pub config_allowed_ips: Vec<String>,
    pub gprc_keys_url: String,
    pub config_dns: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            grpc_port: get_env_var_or_default("VPNAAS_GRPC_PORT", "80"),
            gprc_keys_url: get_env_var_or_default("VPNAAS_GRPC_KEYS_URL", "http://keys:80"),
            config_endpoint: get_env_var("VPNAAS_CONFIG_ENDPOINT"),
            config_pubkey: get_env_var("VPNAAS_CONFIG_PUBKEY"),
            config_dns: get_env_var("VPNAAS_CONFIG_DNS"),
            config_allowed_ips: get_env_var("VPNAAS_CONFIG_ALLOWED_IPS")
                .split_whitespace()
                .map(|s| s.to_owned())
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
