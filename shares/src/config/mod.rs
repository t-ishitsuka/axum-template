pub mod app;

use dotenvy::from_filename;

use app::AppConfig;

///
/// 各種設定を取得する
///
pub fn config(key: &str) -> Config {
    let app = AppConfig::default();

    match key {
        "app" => Config::AppConfig(app),
        "app.env" => Config::String(app.env),
        "app.port" => Config::U16(app.port),
        "app.is_local" => Config::Bool(app.is_local),
        "app.is_development" => Config::Bool(app.is_development),
        "app.is_staging" => Config::Bool(app.is_staging),
        "app.is_production" => Config::Bool(app.is_production),
        "app.is_testing" => Config::Bool(app.is_testing),
        _ => panic!("test"),
    }
}

///
/// 環境変数の読み込み先を調整
///
pub fn load_env() {
    if cfg!(test) || std::env::var("TEST_MODE").is_ok() {
        // テスト
        from_filename("./shares/env/.env.testing").expect(".env.testing file not found.");
    } else if cfg!(debug_assertions) {
        // ローカル、dev, stg
        from_filename("./shares/env/.env").expect(".env file not found.");
    } else {
        // 本番環境ではサーバー環境変数を読む
    }
}

///
/// config 関数に渡したキーにより返却する値が変わるので、戻り値として包括するためのENUM
///
#[derive(Debug)]
pub enum Config {
    // TODO 各種必要な型に合わせて追加実装
    AppConfig(AppConfig),
    String(String),
    U16(u16),
    Bool(bool),
}

// TODO 各種必要な型に合わせて追加実装

///
/// AppConfig への変換
///
impl From<Config> for AppConfig {
    fn from(config: Config) -> Self {
        match config {
            Config::AppConfig(value) => value,
            _ => panic!("aaa"),
        }
    }
}

///
/// String への変換
///
impl From<Config> for String {
    fn from(config: Config) -> Self {
        match config {
            Config::String(value) => value,
            _ => panic!("aaa"),
        }
    }
}

///
/// u16 への変換
///
impl From<Config> for u16 {
    fn from(config: Config) -> Self {
        match config {
            Config::U16(value) => value,
            _ => panic!("aaa"),
        }
    }
}

///
/// Bool への変換
///
impl From<Config> for bool {
    fn from(config: Config) -> Self {
        match config {
            Config::Bool(value) => value,
            _ => panic!("aaa"),
        }
    }
}
