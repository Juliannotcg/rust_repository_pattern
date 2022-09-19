pub mod factories;
pub mod models;
pub mod repositories;
pub mod services;
pub mod database;
pub mod id;

use crate::factories::user_factory::UserFactory;
use crate::services::user_service::{UserService, Service};

fn main() {

    let mut user_factory = UserFactory::new();
    let entity = user_factory.create("Jesus");
    
    let mut user_service = UserService::new();

    let result = user_service.add(&entity);

    match result {
        Ok(success) => success,
        Err(error) => panic!("Error ao salvar usuário: {:?}", error),
    };
}
