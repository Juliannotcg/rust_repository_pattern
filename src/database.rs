use std::sync::{Arc, Mutex};

use crate::services::service::Service;
use crate::services::user_service::UserService;

pub struct Database {
    pub users:  UserService,
}

impl Database {
    
    pub fn new() -> Self {
        
        let connection =
            Arc::new(
                Mutex::new(
                    rusqlite::Connection::open("database_path").unwrap()
                )
            );

        let users = UserService::new(connection.clone());

        Database { users }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_database_new() {
        Database::new();
    }
}  