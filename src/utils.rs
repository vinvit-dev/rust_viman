use std::env;

pub fn get_secret() -> String {
    env::var("SECRET_KEY").expect("env variable SECRET_KEY must be set")
}

pub fn get_secret_jwt() -> String {
    env::var("SECRET_JWT").expect("env variable SECRET_JWT must be set")
}
