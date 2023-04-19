use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use tonic::metadata::MetadataMap;
use tonic::Status;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    exp: usize,
}

pub struct JwtValidator {
    secret_key: String,
}

impl JwtValidator {
    pub fn new(secret_key: String) -> Self {
        JwtValidator { secret_key }
    }

    fn decode(&self, token: &str) -> Result<TokenData<Claims>, Status> {
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| Status::unauthenticated(format!("Invalid JWT token: {}", e)))
    }

    pub fn validate(&self, request_username: &str, metadata: &MetadataMap) -> Result<(), Status> {
        let (name, token) = metadata
            .get("authorization")
            .ok_or_else(|| Status::unauthenticated("Missing authorization header"))?
            .to_str()
            .map_err(|e| Status::unauthenticated(format!("Invalid authorization header: {}", e)))?
            .split_once(" ")
            .ok_or_else(|| Status::unauthenticated("Invalid authorization header"))?;

        if name != "Bearer" {
            return Err(Status::unauthenticated("Missing Bearer token"));
        }

        let jwt = self.decode(token)?.claims;
        if request_username == jwt.username {
            Ok(())
        } else {
            Err(Status::unauthenticated(
                "Username in request does not match username in JWT",
            ))
        }
    }
}
