use std::env;

const DEFAULT_PORT: u16 = 80;

#[derive(Debug)]
pub struct AppConfig {
    pub port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: env::var("APP_PORT")
                .ok()
                .and_then(|port| port.parse::<u16>().ok())
                .unwrap_or(DEFAULT_PORT),
        }
    }
}
