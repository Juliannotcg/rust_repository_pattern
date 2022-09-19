pub mod user;
pub mod model;


#[cfg(test)]
mod tests {
    use super::user::User;

    #[test]
    fn user() {
        let user = User::new(111,
             "test".to_string(),
              0,
               0);

        assert_eq!(user.id(), 111);
        assert_eq!(user.name, "test");
        assert_eq!(user.bank, 0);
        assert_eq!(user.wallet, 0);
    }
}
