use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::Utc;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_identity_session::resource_access_token_claims::ResourceAccessTokenClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_registered_claims::merge_claims::merge_claims;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::no_store::no_store;
use crate::provider_error::ProviderError;
use crate::userinfo_authentication::UserinfoAuthentication;
use crate::userinfo_grant::UserinfoGrant;

const SUBJECT_MEMBER: &str = "sub";

fn refused(challenge: BearerChallenge) -> UserinfoAuthentication {
    UserinfoAuthentication::Refused(challenge.response())
}

pub struct UserinfoEndpoint {
    issuance: TokenIssuance,
    secret_store: Arc<JwksSecretStore>,
}

impl UserinfoEndpoint {
    #[must_use]
    pub fn create(secret_store: Arc<JwksSecretStore>, issuance: TokenIssuance) -> Self {
        Self {
            issuance,
            secret_store,
        }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::UserinfoClaims` when the claims cannot be merged with the subject.
    pub fn answer<TClaims: Serialize>(
        &self,
        grant: &UserinfoGrant,
        claims: &TClaims,
    ) -> Result<Response, ProviderError> {
        merge_claims(
            Map::from_iter([(
                SUBJECT_MEMBER.to_string(),
                Value::String(grant.subject.to_string()),
            )]),
            claims,
        )
        .map_err(ProviderError::UserinfoClaims)
        .map(|members| no_store(Response::json(200, &Value::Object(members))))
    }

    #[must_use]
    pub fn authenticate(&self, request: &Request) -> UserinfoAuthentication {
        let token = match request.inputs.server.authorization().bearer() {
            ControlFlow::Continue(token) => token,
            ControlFlow::Break(challenge) => return refused(challenge),
        };

        match self.secret_store.verify_resource_access_token(
            token.as_str(),
            &[self.issuance.issuer],
            Utc::now(),
        ) {
            JwtVerification::Rejected(_) => refused(BearerChallenge::InvalidToken),
            JwtVerification::Verified(VerifiedJwt {
                claims:
                    ResourceAccessTokenClaims {
                        scope: ScopeList { scopes },
                        subject,
                        ..
                    },
                ..
            }) => match Uuid::try_parse(&subject) {
                Ok(subject) => {
                    UserinfoAuthentication::Authenticated(UserinfoGrant { scopes, subject })
                }
                Err(_) => refused(BearerChallenge::InvalidToken),
            },
        }
    }
}
