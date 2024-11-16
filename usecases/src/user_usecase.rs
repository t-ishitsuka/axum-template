use interfaces::rest::usecases::user_usecase::UserUsecase;
use shaku::Component;

#[derive(Component)]
#[shaku(interface = UserUsecase)]
pub struct UserUsecaseImpl;

impl UserUsecase for UserUsecaseImpl {
    fn find_by_id(&self) -> String {
        "find_by_id".to_string()
    }
}
