use std::env;

pub struct Config {
    pub server_port: String,
    pub wg_server_endpoint: String,
    pub wg_server_pubkey: String,
    pub allowed_ips: Vec<String>,
    pub keys_url: String,
    pub dns_config: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            server_port: get_env_var_or_default("PORT", "80"),
            wg_server_endpoint: get_env_var("WG_SERVER_ENDPOINT"),
            wg_server_pubkey: get_env_var("WG_SERVER_PUBKEY"),
            allowed_ips: get_env_var("ALLOWED_IPS")
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
            dns_config: get_env_var("DNS_CONFIG"),
            keys_url: get_env_var("KEYS_URL"),
        }
    }
}

fn get_env_var(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| panic!("Environment variable {} is unset", var_name))
}

fn get_env_var_or_default(var_name: &str, default_value: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| default_value.to_string())
}
