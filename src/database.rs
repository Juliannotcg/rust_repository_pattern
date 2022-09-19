use cdbc_mssql::MssqlPool;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Configuration {
    connectionstring: String,
}


pub struct Database {
    pub connection:  cdbc::Pool<cdbc_mssql::Mssql>,
}

impl Database {
    
    pub fn new() -> Self {

         let data = fs::read_to_string("./appsettings.json")
        .expect("Unable to read file");

        let connection_string = get_connection_string();

        let value = MssqlPool::connect(&connection_string);
        let connection = value.unwrap();

        Self { connection }
    }
}

fn get_connection_string() -> String {

    let data = fs::read_to_string("./appsettings.json")
        .expect("Unable to read file");

    let configuration: Configuration = serde_json::from_str(&data).expect("JSON was not well-formatted");

    configuration.connectionstring
}