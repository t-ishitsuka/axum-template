#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rstest::rstest;
    use shares::config::{app::AppConfig, load_env};

    #[rstest]
    fn app_config_shows_collect_port() -> Result<()> {
        load_env();

        let app = AppConfig::default();

        assert_eq!(8081, app.port);
        Ok(())
    }
}
