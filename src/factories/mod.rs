pub mod user_factory;

#[cfg(test)]
mod tests {
    use crate::factories::user_factory::UserFactory;

    #[test]
    fn user_factory() {
        let mut user_factory = UserFactory::new();
        let user = user_factory.create("test");

        assert_eq!(user.name, "test");
        assert_eq!(user.bank, 0);
        assert_eq!(user.wallet, 0);
    }
}