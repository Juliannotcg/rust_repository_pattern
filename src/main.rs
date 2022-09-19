pub mod factories;
pub mod models;
pub mod repositories;
pub mod services;
pub mod database;
pub mod id;

use crate::factories::user_factory::UserFactory;
use crate::services::user_service::{UserService, Service};
use value_input::get_input;

fn main() {

    let mut user_factory = UserFactory::new();

    let input: String = get_input("Nome do novo usuário: ");


    let entity = user_factory.create(&input);
    
    let mut user_service = UserService::new();

    let result = user_service.add(&entity);

    match result {
        Ok(_) => print!("Usuário {:?} salvo com sucesso!", &input),
        Err(error) => panic!("Error ao salvar usuário: {:?}", error),
    };
}

mod value_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}
