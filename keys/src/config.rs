use std::env;

pub struct Config {
    pub grpc_port: String,
    pub redis_url: String,
    pub jwks_url: String,
    pub grpc_wg_url: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            grpc_port: get_env_var_or_default("VPNAAS_GRPC_PORT", "80"),
            grpc_wg_url: get_env_var_or_default("VPNAAS_GRPC_WG_URL", "http://wg:80"),
            jwks_url: get_env_var("VPNAAS_JWKS_URL"),
            redis_url: format!(
                "redis://{}:{}@{}:{}/{}",
                get_env_var_or_default("VPNAAS_REDIS_USERNAME", "keys"),
                get_env_var("VPNAAS_REDIS_PASSWORD"),
                get_env_var_or_default("VPNAAS_REDIS_HOSTNAME", "redis"),
                get_env_var_or_default("VPNAAS_REDIS_PORT", "6379"),
                get_env_var_or_default("VPNAAS_REDIS_DATABASE", "0")
            ),
        }
    }
}

fn get_env_var(var_name: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| panic!("Environment variable {} is unset", var_name))
}

fn get_env_var_or_default(var_name: &str, default_value: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| default_value.to_string())
}
