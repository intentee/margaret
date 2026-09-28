use margaret_jwks_keygen::public_jwks::PublicJwks;

use crate::jwks_client_error::JwksClientError;

pub enum PublicJwksPoll {
    Cancelled,
    Failed(JwksClientError),
    Fetched(PublicJwks),
}
