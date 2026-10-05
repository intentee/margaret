use uuid::Uuid;

use margaret_identity_session::client_assertion_claims::ClientAssertionClaims;
use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::client_secret_basic::ClientSecretBasic;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::client_authentication::ClientAuthentication;
use crate::presented_client_authentication::PresentedClientAuthentication;

pub struct OAuthClient {
    pub authentication: ClientAuthentication,
    pub client_id: ClientId,
}

impl OAuthClient {
    #[must_use]
    pub fn presented_to(
        &self,
        issuer: &IssuerIdentifier,
        issued_at: NumericDate,
        expires_at: NumericDate,
    ) -> PresentedClientAuthentication {
        match &self.authentication {
            ClientAuthentication::ClientSecretBasic(secret) => {
                PresentedClientAuthentication::Basic(ClientSecretBasic::authorization(
                    &self.client_id,
                    secret,
                ))
            }
            ClientAuthentication::PrivateKeyJwt(signer) => {
                PresentedClientAuthentication::Assertion(signer.sign_client_assertion(
                    &ClientAssertionClaims {
                        audience: issuer.clone(),
                        client_id: self.client_id.clone(),
                        expires_at,
                        issued_at,
                        jti: Uuid::new_v4(),
                    },
                ))
            }
        }
    }
}
