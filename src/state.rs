use std::collections::HashSet;

#[derive(Clone)]
pub struct State {
    database: deadpool_postgres::Pool,
    generated_aliases : HashSet<String>
}

impl State {
    #[must_use]
    pub fn new(database: deadpool_postgres::Pool) -> Self {
        Self { database, generated_aliases: Default::default() }
    }

    pub async fn database_client(
        &self,
    ) -> Result<deadpool_postgres::Client, deadpool_postgres::PoolError> {
        self.database.get().await
    }
    pub fn add_alias(&mut self, key : &str) {
        self.generated_aliases.insert(key.to_string());
    }
    pub fn find_alias(&self, key : &str) -> bool {
        self.generated_aliases.contains(key)
    }
}
