use shaku::{Component, Interface};

pub trait UserRepository: Interface {
    fn find_by_id(&self) -> String;
}