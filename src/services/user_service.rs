
use crate::models::user::User;
use crate::repositories::repository::Repository;
use crate::repositories::user_repository::UserRepository;
pub use crate::services::service::Service;

pub struct UserService {
    users: UserRepository,
}

impl Service<User> for UserService {

    fn new() -> Self {
        let user_repo = UserRepository::new();

        Self {
            users: user_repo,
        }
    }

    fn add(&mut self, entity: &User) -> Result<(), String> {
        self.users.add(entity)?;
        Ok(())
    }
}