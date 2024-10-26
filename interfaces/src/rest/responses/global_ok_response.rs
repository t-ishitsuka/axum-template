use serde::Deserialize;

#[derive(Deserialize)]
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
