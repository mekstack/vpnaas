use crate::jwt_claims::Claims;

use std::{sync::Arc, time::Duration};

use aliri::jwt;
use aliri_oauth2 as oauth;
use tonic::{metadata::MetadataMap, Status};

pub struct JwtValidator {
    authority: Arc<oauth::Authority>,
}

impl JwtValidator {
    pub async fn new(jwks_url: String) -> Result<Self, reqwest::Error> {
        let authority =
            oauth::Authority::new_from_url(jwks_url, jwt::CoreValidator::default()).await?;
        authority.spawn_refresh(Duration::from_secs(60));

        Ok(JwtValidator {
            authority: std::sync::Arc::new(authority),
        })
    }

    pub fn validate(&self, request_username: &str, metadata: &MetadataMap) -> Result<(), Status> {
        let (name, token_str) = metadata
            .get("authorization")
            .ok_or_else(|| Status::unauthenticated("Missing authorization header"))?
            .to_str()
            .map_err(|e| Status::unauthenticated(format!("Invalid authorization header: {}", e)))?
            .split_once(" ")
            .ok_or_else(|| Status::unauthenticated("Invalid authorization header"))?;

        if name != "Bearer" {
            return Err(Status::unauthenticated("Missing Bearer token"));
        }

        let token = jwt::JwtRef::from_str(token_str);
        let claims: Claims = self
            .authority
            .verify_token(token, &oauth::ScopePolicy::allow_any())
            .map_err(|e| Status::unauthenticated(format!("Token verification failed: {}", e)))?;

        if request_username == claims.username {
            Ok(())
        } else {
            Err(Status::unauthenticated(
                "Username in request does not match username in JWT",
            ))
        }
    }
}
