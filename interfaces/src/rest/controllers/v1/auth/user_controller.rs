use registry::UserModule;
use shaku_axum::Inject;
use usecases::user_usecase::UserUsecase;

///
/// ログインユーザーを取得する
///
pub async fn user(user_usecase: Inject<UserModule, dyn UserUsecase>) -> String {
    user_usecase.find_by_id()
}
