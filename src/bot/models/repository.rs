use sqlx::{Pool, Sqlite};

pub struct Repository {
    pub db: Pool<Sqlite>,
}

impl Repository {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
}
