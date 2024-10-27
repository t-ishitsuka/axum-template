#[derive(utoipa::OpenApi)]
#[openapi(info(
    title = "Axum Example App",
    contact(name = "Aprire", url = "TODO", email = "TODO",),
    description = r#"
## 概要
Axum を用いたWeb アプリケーション BackEnd の構築。
書籍では記載がなかった部分や、気になる個所を追加で試験していく。
"#
))]
pub struct ApiDoc;
