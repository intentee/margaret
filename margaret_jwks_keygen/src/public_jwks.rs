use serde::Serialize;

use margaret_jws_verification::jwk::Jwk;

#[derive(Clone, Serialize)]
pub struct PublicJwks {
    keys: Vec<Jwk>,
}

impl PublicJwks {
    #[must_use]
    pub fn new(keys: Vec<Jwk>) -> Self {
        Self { keys }
    }

    #[must_use]
    pub fn keys(&self) -> &[Jwk] {
        &self.keys
    }
}
