pub trait Repository<TEntity>
{
    /// create a new repository with the connection
    fn new() -> Self;

    /// save the changes, commit the transaction with this.
    //fn save_changes(&mut self)            -> Result<(), String>;

    /// get all entities
    // fn get_all(&self)                     -> Result<Vec<TEntity>, String>;

    // /// get a single entity by id
    // fn get_by_id(&self,  id: &u64)         -> Result<Option<TEntity>, String>;

    /// add an entity to the database
    fn add(&mut self,   entity: &TEntity) -> Result<(), String>;

    // update an entity
    // fn update(&mut self, entity: &TEntity) -> Result<(), String>;

    // /// delete an entity by its id
    // fn delete(&mut self, id: &u64)         -> Result<(), String>;
}