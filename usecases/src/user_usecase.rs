use shaku::{Component, Interface};

#[derive(Component)]
#[shaku(interface = UserUsecase)]
pub struct UserUsecaseImpl;

pub trait UserUsecase: Interface {
    fn find_by_id(&self) -> String;
}

impl UserUsecase for UserUsecaseImpl {
    fn find_by_id(&self) -> String {
        "find_by_id".to_string()
    }
}
