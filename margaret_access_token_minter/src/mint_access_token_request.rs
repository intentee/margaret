use serde::Deserialize;

use margaret_http::request::Request;

use crate::mint_access_token_error::MintAccessTokenError;

#[derive(Deserialize)]
pub struct MintAccessTokenRequest {
    pub refresh_token: String,
}

impl MintAccessTokenRequest {
    /// # Errors
    ///
    /// Returns `MintAccessTokenError::MissingBody` or `MintAccessTokenError::MalformedRequest`.
    pub fn from_request(request: &Request) -> Result<Self, MintAccessTokenError> {
        let body = request
            .inputs
            .json
            .as_ref()
            .ok_or(MintAccessTokenError::MissingBody)?;

        Self::deserialize(body).map_err(|source| MintAccessTokenError::MalformedRequest { source })
    }
}
