use argonautica::{Hasher, Verifier};

use crate::utils::get_secret;

pub struct Password;

impl Password {
    pub fn hash(password_to_hash: String) -> String {
        let hasher = &mut Hasher::default();
        let hashed_password = hasher
            .with_password(password_to_hash)
            .with_secret_key(get_secret())
            .hash()
            .unwrap();

        hashed_password
    }

    pub fn verify(password_to_verify: &String, hashed_password: &String) -> bool {
        let verifier = &mut Verifier::default();
        let result = verifier
            .with_password(password_to_verify)
            .with_hash(hashed_password)
            .with_secret_key(get_secret())
            .verify()
            .unwrap();

        result
    }
}
