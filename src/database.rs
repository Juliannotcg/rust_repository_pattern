use cdbc::{Executor, impl_scan};
use cdbc_mssql::MssqlPool;
use std::sync::{Arc, Mutex};

use crate::services::service::Service;
use crate::services::user_service::UserService;

pub struct Database {
    pub connection:  cdbc::Pool<cdbc_mssql::Mssql>,
}

impl Database {
    
    pub fn new() -> Self {
        let connection = MssqlPool::connect("hmlsql.webmotors.cloud://user_carros:user_carros@hmlsql.webmotors.cloud/DB_WebMotors")?;
        Self { connection }
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