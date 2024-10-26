use serde::Serialize;

///
/// 全アプリケーション共通で成功したというレスポンスを返す時のレスポンス
///
#[derive(Debug, Serialize)]
pub struct GlobalOkResponse {
    message: String,
}

impl Default for GlobalOkResponse {
    fn default() -> Self {
        Self {
            message: "ok".to_string(),
        }
    }
}
