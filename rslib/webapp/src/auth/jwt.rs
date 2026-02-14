use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject (user ID)
    pub username: String, // Username for convenience
    pub exp: i64,         // Expiration time
    pub iat: i64,         // Issued at
    pub session_id: String, // Session ID for revocation
}

impl Claims {
    pub fn new(user_id: i64, username: String, session_id: String, ttl_hours: i64) -> Self {
        let now = Utc::now();
        let expiration = now + Duration::hours(ttl_hours);

        Self {
            sub: user_id.to_string(),
            username,
            exp: expiration.timestamp(),
            iat: now.timestamp(),
            session_id,
        }
    }

    pub fn user_id(&self) -> Result<i64> {
        self.sub.parse().map_err(|e| anyhow::anyhow!("Invalid user ID in token: {}", e))
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtManager {
    pub fn new(secret: &str) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 60; // 60 seconds leeway for clock skew

        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            validation,
        }
    }

    pub fn generate_token(&self, claims: &Claims) -> Result<String> {
        let header = Header::new(Algorithm::HS256);
        encode(&header, claims, &self.encoding_key)
            .map_err(|e| anyhow::anyhow!("Failed to generate JWT: {}", e))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map_err(|e| anyhow::anyhow!("Failed to verify JWT: {}", e))?;

        if token_data.claims.is_expired() {
            return Err(anyhow::anyhow!("Token has expired"));
        }

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_creation() {
        let claims = Claims::new(123, "testuser".to_string(), "session_abc".to_string(), 24);
        
        assert_eq!(claims.sub, "123");
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.session_id, "session_abc");
        assert!(!claims.is_expired());
        assert_eq!(claims.user_id().unwrap(), 123);
    }

    #[test]
    fn test_jwt_generation_and_verification() {
        let manager = JwtManager::new("test_secret_key");
        let claims = Claims::new(456, "alice".to_string(), "sess_xyz".to_string(), 1);

        let token = manager.generate_token(&claims).unwrap();
        assert!(!token.is_empty());

        let verified = manager.verify_token(&token).unwrap();
        assert_eq!(verified.sub, "456");
        assert_eq!(verified.username, "alice");
        assert_eq!(verified.session_id, "sess_xyz");
        assert_eq!(verified.user_id().unwrap(), 456);
    }

    #[test]
    fn test_invalid_token() {
        let manager = JwtManager::new("test_secret");
        let result = manager.verify_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_secret() {
        let manager1 = JwtManager::new("secret1");
        let manager2 = JwtManager::new("secret2");

        let claims = Claims::new(789, "bob".to_string(), "session_123".to_string(), 1);
        let token = manager1.generate_token(&claims).unwrap();

        let result = manager2.verify_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_expired_token() {
        let manager = JwtManager::new("test_secret");
        
        // Create expired claims (0 hours TTL)
        let mut claims = Claims::new(999, "expired_user".to_string(), "sess_exp".to_string(), 0);
        claims.exp = Utc::now().timestamp() - 3600; // 1 hour ago

        let token = manager.generate_token(&claims).unwrap();
        let result = manager.verify_token(&token);
        
        // Should fail either with "expired" or JWT decode error
        assert!(result.is_err());
    }
}
