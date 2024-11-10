use argon2::{Argon2, PasswordHash, PasswordVerifier};
use password_hash::{PasswordHasher, SaltString};
use rand_core::OsRng;

use crate::errors::AppError;

pub fn gen_password_hash(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let pwd_hash_res = argon2.hash_password(password.as_bytes(), &salt);

    match pwd_hash_res {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => Err(AppError::PasswordHashError(format!("{}", e))),
    }
}

pub fn verify_password(password: &str, hash_value: &str) -> Result<bool, AppError> {
    let pwd_hash_res = PasswordHash::new(hash_value);

    match pwd_hash_res {
        Ok(pwd_hash) => {
            let argon2 = Argon2::default();
            match argon2.verify_password(password.as_bytes(), &pwd_hash) {
                Ok(()) => Ok(true),
                Err(_) => Ok(false),
            }
        }
        Err(e) => Err(AppError::PasswordHashError(format!("{}", e))),
    }
}
