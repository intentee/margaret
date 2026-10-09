use std::collections::BTreeSet;
use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_parsing::ScopeParsing;

use crate::acquired_token::AcquiredToken;
use crate::client_credentials::ClientCredentials;
use crate::client_credentials_error::ClientCredentialsError;
use crate::resource_grant::ResourceGrant;

pub struct ResourceCredentials {
    credentials: ClientCredentials,
    target: TokenTarget,
}

impl ResourceCredentials {
    /// # Errors
    ///
    /// Returns `ClientCredentialsError::MalformedGrantScope` when the grant requests a scope that
    /// is not a scope token.
    pub fn create(
        server: Arc<AuthorizationServerClient>,
        ResourceGrant { audience, scopes }: ResourceGrant,
    ) -> Result<Self, ClientCredentialsError> {
        Ok(Self {
            credentials: ClientCredentials::create(server),
            target: TokenTarget {
                audience: TargetAudience::Audience(audience.to_string()),
                scopes: scopes
                    .iter()
                    .map(|scope| match Scope::parse(scope) {
                        ScopeParsing::Accepted(parsed) => Ok(parsed),
                        ScopeParsing::Rejected(rejection) => {
                            Err(ClientCredentialsError::MalformedGrantScope { rejection, scope })
                        }
                    })
                    .collect::<Result<BTreeSet<Scope>, ClientCredentialsError>>()?,
            },
        })
    }

    pub async fn access_token(&self) -> AcquiredToken {
        self.credentials.access_token(&self.target).await
    }
}
