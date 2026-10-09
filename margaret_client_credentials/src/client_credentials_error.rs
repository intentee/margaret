use thiserror::Error;

use margaret_oauth_vocabulary::scope_rejection::ScopeRejection;

#[derive(Debug, Error)]
pub enum ClientCredentialsError {
    #[error("the resource grant requests the scope '{scope}', which is malformed: {rejection}")]
    MalformedGrantScope {
        rejection: ScopeRejection,
        scope: &'static str,
    },
}
