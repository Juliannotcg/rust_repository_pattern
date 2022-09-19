
#[derive(Debug)]
pub struct User {
    id: u64,
    pub name: String,
}

impl User {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn new(id: u64, name: String) -> Self {
        Self { id, name}
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

