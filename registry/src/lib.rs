use std::sync::Arc;

use shaku::module;
use usecases::user_usecase::UserUsecaseImpl;

module! {
    pub UserModule {
        components = [UserUsecaseImpl],
        providers = []
    }
}

pub struct AppRegistry {
    pub user_module: Arc<UserModule>,
}

impl AppRegistry {
    pub fn new() -> Self {
        let user_module = Arc::new(UserModule::builder().build());

        Self {
            user_module: user_module,
        }
    }
}
