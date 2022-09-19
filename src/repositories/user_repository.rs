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

        match conn.execute(query) {
                Ok(success) => success,
                Err(error) => panic!("Problem opening database: {:?}", error),
            };


        Ok(())
    }
}