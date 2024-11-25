use domains::repositories::user_repository::UserRepository;
use shaku::{Interface, Provider};

#[derive(Provider)]
#[shaku(interface = UserUsecase)]
pub struct UserUsecaseImpl {
    #[shaku(provide)]
    user_persistence: Box<dyn UserRepository>,
}

pub trait UserUsecase: Interface {
    fn find_by_id(&self) -> String;
}

impl UserUsecase for UserUsecaseImpl {
    fn find_by_id(&self) -> String {
        self.user_persistence.find_by_id()
    }
}
