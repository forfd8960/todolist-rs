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

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{gen_password_hash, verify_password};

    #[test]
    fn test_hash_verify_password() -> Result<()> {
        let pwd_hash = gen_password_hash("abc132569")?;
        let result = verify_password("abc132569", &pwd_hash)?;
        assert!(result);
        Ok(())
    }

    #[test]
    fn test_bad_hash() -> Result<()> {
        let pwd_hash1 = gen_password_hash("def132569")?;
        let verify_res = verify_password("abc132569", &pwd_hash1)?;
        assert!(!verify_res);

        let verify_res = verify_password("def132569", "test-hash");
        assert!(verify_res.is_err());
        Ok(())
    }
}
