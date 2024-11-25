use std::sync::Arc;

use axum::extract::FromRef;
use infrastructures::persistences::user::UserPersistence;
use shaku::module;
use usecases::user_usecase::UserUsecaseImpl;

module! {
    pub UserModule {
        components = [],
        providers = [UserUsecaseImpl, UserPersistence]
    }
}

#[derive(Clone)]
pub struct AppState {
    pub user_module: Arc<UserModule>,
}

impl AppState {
    pub fn new() -> Self {
        let user_module = Arc::new(UserModule::builder().build());

        Self { user_module }
    }
}

impl FromRef<AppState> for Arc<UserModule> {
    fn from_ref(app_state: &AppState) -> Arc<UserModule> {
        app_state.user_module.clone()
    }
}
