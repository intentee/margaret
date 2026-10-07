use margaret_http::server_origin::ServerOrigin;

use crate::provider_error::ProviderError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderEndpoints {
    pub authorization: &'static str,
    pub introspection: &'static str,
    pub issuer_origin: &'static str,
    pub jwks: &'static str,
    pub revocation: &'static str,
    pub token: &'static str,
    pub userinfo: &'static str,
}

impl ProviderEndpoints {
    /// # Errors
    ///
    /// Returns `ProviderError::IssuerNotServed` when the issuer is not served at the origin of
    /// the server.
    pub fn served_by(&self, server: &ServerOrigin) -> Result<(), ProviderError> {
        let server_origin = server.origin.ascii_serialization();

        if server_origin == self.issuer_origin {
            Ok(())
        } else {
            Err(ProviderError::IssuerNotServed {
                issuer_origin: self.issuer_origin,
                server_origin,
            })
        }
    }
}
