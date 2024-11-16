use shaku::Interface;

pub trait UserUsecase: Interface {
    fn find_by_id(&self) -> String;
}
