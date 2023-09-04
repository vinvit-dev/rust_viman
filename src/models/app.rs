use diesel::PgConnection;
use std::sync::Mutex;

pub struct AppState {
    pub db: Mutex<PgConnection>,
}
