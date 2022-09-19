use cdbc_mssql::MssqlPool;

pub struct Database {
    pub connection:  cdbc::Pool<cdbc_mssql::Mssql>,
}

impl Database {
    
    pub fn new() -> Self {

        let value = MssqlPool::connect("mssql://SA:TestPass!123456@localhost:1433/test");
        let connection = value.unwrap();

        Self { connection }
    }
}
