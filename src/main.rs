pub mod factories;
pub mod models;
pub mod repositories;
pub mod services;
pub mod database;
pub mod id;

use crate::database::Database;
use crate::factories::user_factory::UserFactory;
use crate::services::user_service::UserService;


fn main() {
    
    let mut db = Database::new();
    let mut user_factory = UserFactory::new();

    let mut user_factory = UserService::add();

    

    let user = user_factory.create("ddjerqq");

    db.add(&user).unwrap();


    print!("{}", user.name);
}
