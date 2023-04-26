use std::env;

pub struct Config {
    pub server_port: String,
    pub redis_url: String,
    pub jwt_secret_key: String,
    pub wg_server_url: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            server_port: get_env_var_or_default("PORT", "80"),
            jwt_secret_key: get_env_var("JWT_SECRET_KEY"),
            wg_server_url: get_env_var("WG_SERVER_URL"),
            redis_url: format!(
                "redis://{}:{}@{}:{}",
                get_env_var("REDIS_USERNAME"),
                get_env_var("REDIS_PASSWORD"),
                get_env_var("REDIS_HOSTNAME"),
                get_env_var_or_default("REDIS_PORT", "6379")
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
