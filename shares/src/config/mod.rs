pub mod app;

use dotenvy::from_filename;

use app::AppConfig;

///
/// 各種設定を取得する
///
pub fn config(key: &str) -> Config {
    load_env();

    let app = AppConfig::default();

    match key {
        "app" => Config::AppConfig(app),
        "app.port" => Config::U16(app.port),
        _ => panic!("test"),
    }
}

///
/// 環境変数の読み込み先を調整
///
pub fn load_env() {
    if cfg!(test) {
        from_filename("./shares/env/.env.testing").expect(".env.testing file not found.");
    } else if cfg!(debug_assertions) {
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
