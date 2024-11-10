use std::ops::Deref;

use anyhow::Result;
use jwt_simple::prelude::*;

use crate::{errors::AppError, models::user::User};

const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
const JWT_ISS: &str = "todolist-server";
const JWT_AUD: &str = "todolist-client";

pub struct Authorization {
    pub encode_key: EncodingKey,
    pub decode_key: DecodingKey,
}

impl Authorization {
    pub fn new(encode_pem: &str, decode_pem: &str) -> Result<Self, AppError> {
        let enc_key = EncodingKey::load(encode_pem)?;
        let dec_key = DecodingKey::load(decode_pem)?;
        Ok(Self {
            encode_key: enc_key,
            decode_key: dec_key,
        })
    }

    pub fn sign(&self, user: impl Into<User>) -> Result<String, AppError> {
        self.encode_key.sign(user)
    }

    pub fn verify_user(&self, token: &str) -> Result<User, AppError> {
        self.decode_key.verify(token)
    }
}

pub struct EncodingKey(Ed25519KeyPair);
pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }

    pub fn sign(&self, user: impl Into<User>) -> Result<String, AppError> {
        let u: User = user.into();
        let claims = Claims::with_custom_claims(u, Duration::from_secs(JWT_DURATION));
        let claims = claims.with_issuer(JWT_ISS).with_audience(JWT_AUD);

        Ok(self.0.sign(claims)?)
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    pub fn verify(&self, token: &str) -> Result<User, AppError> {
        let options = jwt_simple::common::VerificationOptions {
            allowed_issuers: Some(HashSet::from([JWT_ISS.to_string()])),
            allowed_audiences: Some(HashSet::from([JWT_AUD.to_string()])),
            ..Default::default()
        };

        let claims = self.0.verify_token::<User>(token, Some(options))?;
        Ok(claims.custom)
    }
}

impl Deref for EncodingKey {
    type Target = Ed25519KeyPair;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for DecodingKey {
    type Target = Ed25519PublicKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_jwt() -> Result<()> {
        let encoding_key = EncodingKey::load(r#""#)?;
        let decoding_key = DecodingKey::load(r#""#)?;

        let user1 = User {
            id: 1,
            username: "<NAME>".to_string(),
            email: "<EMAIL>".to_string(),
            password_hash: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let u = user1.clone();
        let token = encoding_key.sign(user1)?;
        let user2 = decoding_key.verify(&token)?;
        assert_eq!(u, user2);
        Ok(())
    }
}
