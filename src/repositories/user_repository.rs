use std::sync::{Arc, Mutex};


use crate::database::Database;
use crate::models::model::Model;
use crate::models::user::User;
pub use crate::repositories::repository::Repository;
use cdbc::crud::CRUD;

pub struct UserRepository {
    db: Database,
}

impl Repository<User> for UserRepository {
    
    fn new() -> Self {
        let db = Database::new();
        Self { db }
    }

    fn save_changes(&mut self) -> Result<(), String> {
        self.db
            .lock()
            .map_err(|e| e.to_string())?
            .borrow_mut()
            .transaction()
            .and_then(|transaction| transaction.commit())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn get_all(&self) -> Result<Vec<User>, String> {
        let mut con =
            self.connection
                .lock()
                .map_err(|e| e.to_string())?;

        let mut stmt = con
            .borrow_mut()
            .prepare("SELECT * FROM users")
            .map_err(|e| e.to_string())?;

        let entities = stmt.query_map([], |row| {
            if let Ok(entity) = User::from_row(row) {
                Ok(entity)
            } else {
                Err(rusqlite::Error::QueryReturnedNoRows)
            }
        }).map_err(|e| e.to_string())?;

        Ok(entities.flatten().collect())
    }

    fn get_by_id(&self, id: &u64) -> Result<Option<User>, String> {
        let mut con = self.connection
            .lock()
            .map_err(|e| e.to_string())?;

        let mut stmt = con
                .borrow_mut()
                .prepare("SELECT * FROM users WHERE id=?1")
                .map_err(|e| e.to_string())?;

        let mut result = stmt.query_map([id], |row| {
            Ok(User::from_row(row))
        }).map_err(|e| e.to_string())?;

        if let Some(user) = result.next() {
            return if let Ok(Ok(user)) = user {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }
        Ok(None)
    }

    fn add(&mut self, entity: &User) -> Result<(), String> {

        let pool = self.db.connection;

        CRUD::insert(&pool, entity.clone());




        Ok(())
    }

    fn update(&mut self, entity: &User) -> Result<(), String> {
        
        self.db.connection.exe
            .lock()
            .map_err(|e| e.to_string())?
            .borrow_mut()
            .execute("UPDATE Users
            SET name=?2, wallet=?3, bank=?4
            WHERE id=?1;",
            params![entity.id(), entity.name, entity.wallet, entity.bank])
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn delete(&mut self, id: &u64) -> Result<(), String> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .borrow_mut()
            .execute("DELETE FROM Users WHERE id=?1;",
                     params![id])
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}