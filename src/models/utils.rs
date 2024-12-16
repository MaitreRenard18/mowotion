use rand::distributions::{Alphanumeric, DistString};
use std::hash::{DefaultHasher, Hash, Hasher};

pub async fn hash_password(password: &str) -> String {
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    hasher.finish().to_string()
}

pub async fn generate_session_token() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 64)
}

pub async fn generate_csrf_token() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 64)
}
