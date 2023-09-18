use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct Database {
    pub connection: Pool<Postgres>,
}
