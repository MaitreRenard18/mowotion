use rand::distributions::{Alphanumeric, DistString};
use std::hash::{DefaultHasher, Hash, Hasher};
use chrono::{NaiveDate, Utc};

pub async fn hash_password(password: &str) -> String {
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    hasher.finish().to_string()
}

pub async fn generate_session_token() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 64)
}

pub async fn get_expire_date() -> NaiveDate {
    Utc::now()
        .naive_utc()
        .date()
        .checked_add_days(chrono::Days::new(7)) // Ajoute 7 jours
        .expect("Erreur de calcul de date")
}

pub async fn get_current_date() -> NaiveDate {
    Utc::now().naive_utc().date()
}
