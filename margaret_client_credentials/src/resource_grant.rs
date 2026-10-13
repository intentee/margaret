use std::collections::BTreeSet;

use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_parsing::ScopeParsing;

use crate::client_credentials_error::ClientCredentialsError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceGrant {
    pub audience: &'static str,
    pub scopes: &'static [&'static str],
}

impl ResourceGrant {
    /// # Errors
    ///
    /// Returns `ClientCredentialsError::MalformedGrantScope` when the grant requests a scope that
    /// is not a scope token.
    pub fn target(self) -> Result<TokenTarget, ClientCredentialsError> {
        self.scopes
            .iter()
            .map(|scope| match Scope::parse(scope) {
                ScopeParsing::Accepted(parsed) => Ok(parsed),
                ScopeParsing::Rejected(rejection) => {
                    Err(ClientCredentialsError::MalformedGrantScope { rejection, scope })
                }
            })
            .collect::<Result<BTreeSet<Scope>, ClientCredentialsError>>()
            .map(|scopes| TokenTarget {
                audience: TargetAudience::Audience(self.audience.to_string()),
                scopes,
            })
    }
}

#[cfg(test)]
mod tests {
    use margaret_authorization_server_client::target_audience::TargetAudience;
    use margaret_oauth_vocabulary::scope::Scope;
    use margaret_oauth_vocabulary::scope_rejection::ScopeRejection;

    use super::ResourceGrant;
    use crate::client_credentials_error::ClientCredentialsError;

    #[test]
    fn targets_the_audience_of_the_grant_with_its_scopes() {
        let target = ResourceGrant {
            audience: "attachments",
            scopes: &["files:write", "files:read"],
        }
        .target()
        .expect("the grant scopes are scope tokens");

        assert_eq!(
            target.audience,
            TargetAudience::Audience("attachments".to_string())
        );
        assert_eq!(
            target
                .scopes
                .iter()
                .map(Scope::as_str)
                .collect::<Vec<&str>>(),
            ["files:read", "files:write"]
        );
    }

    #[test]
    fn rejects_a_grant_of_a_malformed_scope() {
        assert!(matches!(
            ResourceGrant {
                audience: "attachments",
                scopes: &["files read"],
            }
            .target(),
            Err(ClientCredentialsError::MalformedGrantScope {
                rejection: ScopeRejection::Character,
                scope: "files read",
            })
        ));
    }
}
