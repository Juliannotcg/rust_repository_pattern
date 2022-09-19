use std::sync::{Arc, Mutex};

use crate::database::Database;
use crate::models::user::User;
pub use crate::repositories::repository::Repository;
use cdbc::Executor;

pub struct UserRepository {
    db: Database,
}

impl Repository<User> for UserRepository {
    
    fn new() -> Self {
        let db = Database::new();
        Self { db }
    }

    fn add(&mut self, entity: &User) -> Result<(), String> {

        let pool = &self.db.connection;

        let id = entity.id();
        let name = &entity.name;

        let query: &str = &format!("INSERT INTO [user] (id, name) values ({id}, '{name}')");

        let mut conn = pool.acquire().unwrap();

        let resultQuery = match conn.execute(query) {
                Ok(file) => file,
                Err(error) => panic!("Problem opening database: {:?}", error),
            };

        print!("Linhas afetadas: {}", resultQuery.rows_affected());

        Ok(())
    }
}