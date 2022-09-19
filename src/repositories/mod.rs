pub mod repository;
pub mod user_repository;


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::repository::Repository;
    use super::user_repository::UserRepository;

    #[test]
    fn user_repository() {
        let con = rusqlite::Connection::open_in_memory().unwrap();
        let _user_repository = UserRepository::new(Arc::new(Mutex::new(con)));
    }
}
