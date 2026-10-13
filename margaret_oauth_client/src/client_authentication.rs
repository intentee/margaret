use std::sync::Arc;

use uuid::Uuid;

use margaret_identity_session::client_assertion_claims::ClientAssertionClaims;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_oauth_vocabulary::client_secret::ClientSecret;
use margaret_oauth_vocabulary::client_secret_basic::client_secret_basic;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::presented_client_authentication::PresentedClientAuthentication;

pub enum ClientAuthentication {
    ClientSecretBasic(ClientSecret),
    PrivateKeyJwt(Arc<JwksSecretHolder>),
}

impl ClientAuthentication {
    #[must_use]
    pub fn presented_to(
        &self,
        issuer: &'static str,
        client_id: &'static str,
        issued_at: NumericDate,
        expires_at: NumericDate,
    ) -> PresentedClientAuthentication {
        match self {
            Self::ClientSecretBasic(secret) => {
                PresentedClientAuthentication::Basic(client_secret_basic(client_id, secret))
            }
            Self::PrivateKeyJwt(secrets) => PresentedClientAuthentication::Assertion(
                secrets.get().current().sign_json(
                    &ClientAssertionClaims {
                        audience: issuer,
                        client_id,
                        expires_at,
                        issued_at,
                        jti: Uuid::new_v4(),
                    }
                    .to_payload(),
                    JwtType::ClientAuthentication,
                ),
            ),
        }
    }
}
