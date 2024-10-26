pub mod app;

use dotenvy::dotenv;

use app::AppConfig;

///
/// 各種設定を取得する
///
pub fn config(key: &str) -> Config {
    dotenv().expect(".env file not fount.");

    let app = AppConfig::default();

    match key {
        "app" => Config::AppConfig(app),
        "app.port" => Config::U16(app.port),
        _ => panic!("test"),
    }
}

///
/// config 関数に渡したキーにより返却する値が変わるので、戻り値として包括するためのENUM
///
#[derive(Debug)]
pub enum Config {
    // TODO 各種必要な型に合わせて追加実装
    AppConfig(AppConfig),
    U16(u16),
}

// TODO 各種必要な型に合わせて追加実装

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
