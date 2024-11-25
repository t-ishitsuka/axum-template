use registry::UserModule;
use shaku_axum::InjectProvided;
use usecases::user_usecase::UserUsecase;

///
/// ログインユーザーを取得する
///
pub async fn user(user_usecase: InjectProvided<UserModule, dyn UserUsecase>) -> String {
    user_usecase.find_by_id()
}
