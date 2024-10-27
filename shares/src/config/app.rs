use std::env;

const DEFAULT_ENV: &str = "local";
const DEFAULT_PORT: u16 = 80;

#[derive(Debug)]
pub struct AppConfig {
    pub env: String,
    pub port: u16,
    pub is_local: bool,
    pub is_development: bool,
    pub is_staging: bool,
    pub is_production: bool,
    pub is_testing: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        let app_env = env::var("APP_ENV").ok().unwrap_or(DEFAULT_ENV.into());

        Self {
            env: app_env.clone(),
            port: env::var("APP_PORT")
                .ok()
                .and_then(|port| port.parse::<u16>().ok())
                .unwrap_or(DEFAULT_PORT),
            is_local: app_env.eq(DEFAULT_ENV.into()),
            is_development: app_env.eq("development".into()),
            is_staging: app_env.eq("staging".into()),
            is_production: app_env.eq("production".into()),
            is_testing: app_env.eq("testing".into()),
        }
    }
}
