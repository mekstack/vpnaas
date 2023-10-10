use aliri_clock::UnixTime;
use aliri_oauth2::Scope;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(flatten)]
    pub basic: aliri::jwt::BasicClaims,
    pub username: String,
    pub scope: Scope,
}

impl aliri::jwt::CoreClaims for Claims {
    #[inline]
    fn nbf(&self) -> Option<UnixTime> {
        self.basic.nbf()
    }
    #[inline]
    fn exp(&self) -> Option<UnixTime> {
        self.basic.exp()
    }
    #[inline]
    fn aud(&self) -> &aliri::jwt::Audiences {
        self.basic.aud()
    }
    #[inline]
    fn iss(&self) -> Option<&aliri::jwt::IssuerRef> {
        self.basic.iss()
    }
    #[inline]
    fn sub(&self) -> Option<&aliri::jwt::SubjectRef> {
        self.basic.sub()
    }
}

impl aliri_oauth2::HasScope for Claims {
    #[inline]
    fn scope(&self) -> &Scope {
        &self.scope
    }
}
