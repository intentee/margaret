use std::collections::BTreeSet;
use std::sync::Arc;

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::request::Request;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response::Response;
use margaret_identity_session::resource_access_token_claims::ResourceAccessTokenClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_registered_claims::audience::Audience;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;

use crate::no_store::no_store;
use crate::provider_error::ProviderError;
use crate::userinfo_answer::UserinfoAnswer;
use crate::userinfo_authentication::UserinfoAuthentication;
use crate::userinfo_grant::UserinfoGrant;

const SUBJECT_MEMBER: &str = "sub";

fn refused(challenge: BearerChallenge) -> UserinfoAuthentication {
    UserinfoAuthentication::Refused(challenge.response())
}

pub struct UserinfoEndpoint {
    audiences: BTreeSet<Audience>,
    secret_store: Arc<JwksSecretStore>,
}

impl UserinfoEndpoint {
    #[must_use]
    pub fn create(
        secret_store: Arc<JwksSecretStore>,
        issuance: &dyn DeclaresTokenIssuance,
    ) -> Self {
        Self {
            audiences: BTreeSet::from([Audience::from(&issuance.token_issuance().issuer)]),
            secret_store,
        }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::UserinfoClaimsSerialization` when the claims cannot be serialized.
    pub fn answer<TClaims: Serialize>(
        &self,
        grant: &UserinfoGrant,
        claims: &TClaims,
    ) -> Result<UserinfoAnswer, ProviderError> {
        let Value::Object(mut members) =
            serde_json::to_value(claims).map_err(ProviderError::UserinfoClaimsSerialization)?
        else {
            return Ok(UserinfoAnswer::ClaimsNotAnObject);
        };

        if members.contains_key(SUBJECT_MEMBER) {
            return Ok(UserinfoAnswer::CollidingSubject);
        }

        members.insert(
            SUBJECT_MEMBER.to_string(),
            Value::String(grant.subject.to_string()),
        );

        Ok(UserinfoAnswer::Answered(no_store(Response::json(
            200,
            &Value::Object(members),
        ))))
    }

    /// # Errors
    ///
    /// Returns `ProviderError::Signing` when the signing secret is not available yet.
    pub fn authenticate(&self, request: &Request) -> Result<UserinfoAuthentication, ProviderError> {
        let token = match request.inputs.server.authorization() {
            RequestAuthorization::Absent | RequestAuthorization::OtherScheme => {
                return Ok(refused(BearerChallenge::MissingCredentials));
            }
            RequestAuthorization::Bearer(token) => token,
            RequestAuthorization::Malformed => return Ok(refused(BearerChallenge::InvalidRequest)),
        };

        Ok(
            match self
                .secret_store
                .verify_resource_access_token(token.as_str(), &self.audiences, Utc::now())
                .map_err(ProviderError::Signing)?
            {
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
            },
        )
    }
}
