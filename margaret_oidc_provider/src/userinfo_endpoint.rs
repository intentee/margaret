use std::ops::ControlFlow;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_identity_session::resource_access_token_claims::ResourceAccessTokenClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::answers_userinfo_grants::AnswersUserinfoGrants;
use crate::provides_userinfo_claims::ProvidesUserinfoClaims;
use crate::typed_userinfo_answer::TypedUserinfoAnswer;
use crate::userinfo_authentication::UserinfoAuthentication;
use crate::userinfo_grant::UserinfoGrant;

fn refused(challenge: BearerChallenge) -> UserinfoAuthentication {
    UserinfoAuthentication::Refused(challenge.response())
}

pub struct UserinfoEndpoint {
    answer: Box<dyn AnswersUserinfoGrants>,
    issuance: TokenIssuance,
    secret_store: Arc<JwksSecretStore>,
}

impl UserinfoEndpoint {
    #[must_use]
    pub fn create<TProvider: ProvidesUserinfoClaims>(
        secret_store: Arc<JwksSecretStore>,
        issuance: TokenIssuance,
        claims: Arc<TProvider>,
    ) -> Self {
        Self {
            answer: Box::new(TypedUserinfoAnswer { provider: claims }),
            issuance,
            secret_store,
        }
    }

    fn authenticate(&self, request: &Request) -> UserinfoAuthentication {
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

#[async_trait]
impl HeadHandler for UserinfoEndpoint {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        match self.authenticate(request) {
            UserinfoAuthentication::Authenticated(grant) => self
                .answer
                .answered(&grant)
                .await
                .map(ResponseContinuation::from)
                .map_err(|error| HandlerError::consumer(anyhow::Error::from(error))),
            UserinfoAuthentication::Refused(refusal) => Ok(ResponseContinuation::from(refusal)),
        }
    }
}
