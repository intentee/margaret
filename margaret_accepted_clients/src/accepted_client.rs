use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_registered_claims::audience::Audience;

use crate::accepted_client_authentication::AcceptedClientAuthentication;
use crate::authorization_code_grant::AuthorizationCodeGrant;
use crate::client_credentials_grant::ClientCredentialsGrant;
use crate::code_grant_policy::CodeGrantPolicy;
use crate::confidential_privileges::ConfidentialPrivileges;
use crate::non_empty_set::NonEmptySet;
use crate::refresh_token_grant::RefreshTokenGrant;
use crate::token_exchange_grant::TokenExchangeGrant;

pub struct AcceptedClient {
    pub authentication: AcceptedClientAuthentication,
    pub authorization_code: AuthorizationCodeGrant,
    pub client_id: ClientId,
    pub resources: NonEmptySet<Audience>,
    pub token_exchange: TokenExchangeGrant,
}

impl AcceptedClient {
    #[must_use]
    pub fn grants(&self, grant_type: GrantType) -> bool {
        match grant_type {
            GrantType::AuthorizationCode => {
                matches!(self.authorization_code, AuthorizationCodeGrant::Granted(_))
            }
            GrantType::ClientCredentials => matches!(
                self.authentication,
                AcceptedClientAuthentication::ClientSecretBasic {
                    privileges: ConfidentialPrivileges {
                        client_credentials: ClientCredentialsGrant::Granted { .. },
                        ..
                    },
                    ..
                }
            ),
            GrantType::RefreshToken => matches!(
                self.authorization_code,
                AuthorizationCodeGrant::Granted(CodeGrantPolicy {
                    refresh: RefreshTokenGrant::Granted,
                    ..
                })
            ),
            GrantType::TokenExchange => self.token_exchange == TokenExchangeGrant::Granted,
        }
    }
}
