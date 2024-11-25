use domains::repositories::user_repository::UserRepository;
use shaku::Provider;

#[derive(Provider)]
#[shaku(interface = UserRepository)]
pub struct UserPersistence;

impl UserRepository for UserPersistence {
    fn find_by_id(&self) -> String {
        "find_by_id in user persistence".to_string()
    }
}
