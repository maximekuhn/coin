use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use crate::auth::password::Password;

pub fn hash_password(p: Password) -> Result<Vec<u8>, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(p.value().as_bytes(), &salt)?
        .to_string()
        .as_bytes()
        .to_vec())
}

pub fn verify_password(p: String, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(p.as_bytes(), &parsed_hash)
        .is_ok())
}
