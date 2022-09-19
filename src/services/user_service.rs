use std::sync::{Arc, Mutex};


use crate::models::user::User;
use crate::repositories::repository::Repository;
use crate::repositories::user_repository::UserRepository;
pub use crate::services::service::Service;

pub struct UserService {
    users: UserRepository,
}

impl Service<User> for UserService {

    fn new(connection: Arc<Mutex<Connection>>) -> Self {
        let user_repo = UserRepository::new(connection.clone());

        Self {
            users: user_repo,
        }
    }

    fn get_all(&self) -> Result<Vec<User>, String> {
        let users = self.users.get_all()?;
        Ok(users)
    }

    fn get_by_id(&self, id: &u64) -> Result<Option<User>, String> {
        let user = self.users.get_by_id(id)?;
        return Ok(user)
    }

    fn add(&mut self, entity: &User) -> Result<(), String> {

        self.users.add(entity)?;
        self.users.save_changes()?;

        
        Ok(())
    }

    fn update(&mut self, entity: &User) -> Result<(), String> {
        self.users.update(entity)?;

        self.users.save_changes()?;
        Ok(())
    }

    fn delete(&mut self, id: &u64) -> Result<(), String> {
        self.users.delete(id)?;
        self.users.save_changes()?;
        Ok(())
    }
}