use std::collections::BTreeSet;
use std::iter;
use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::Utc;
use oauth2::CsrfToken;
use oauth2::RedirectUrl;
use oauth2::TokenResponse;
use serde::de::DeserializeOwned;

use margaret_authorization_server_client::authorization_request::AuthorizationRequest;
use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::authorization_url::AuthorizationUrl;
use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::userinfo_outcome::UserinfoOutcome;
use margaret_http::redirect::Redirect;
use margaret_http::request::Request;
use margaret_identity_session::sign_in_transaction_claims::SignInTransactionClaims;
use margaret_identity_session::sign_in_transaction_lifetime_secs::SIGN_IN_TRANSACTION_LIFETIME_SECS;
use margaret_issuer_key_set::issuer_verification::IssuerVerification;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwt_verification::attribute_serialized_jwt::attribute_serialized_jwt;
use margaret_jwt_verification::expected_audience::ExpectedAudience;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::sign_in_transaction_profile::SignInTransactionProfile;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_oauth_vocabulary::code_challenge::CodeChallenge;
use margaret_oauth_vocabulary::code_verifier::CodeVerifier;
use margaret_oauth_vocabulary::openid_scope::OPENID_SCOPE;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::equal_in_constant_time::equal_in_constant_time;
use margaret_token_digest::random_token::random_token;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::callback_code::callback_code;
use crate::id_token_claims::IdTokenClaims;
use crate::id_token_fields::IdTokenFields;
use crate::sign_in_beginning::SignInBeginning;
use crate::sign_in_completion::SignInCompletion;
use crate::sign_in_flow_error::SignInFlowError;
use crate::sign_in_refusal::SignInRefusal;
use crate::signed_in::SignedIn;
use crate::transaction_cookie::TransactionCookie;
use crate::userinfo_claims::UserinfoClaims;
use crate::userinfo_fetch::UserinfoFetch;

fn rejected_id_token<TIdClaims>(rejection: JwtRejection) -> SignInCompletion<TIdClaims> {
    SignInCompletion::Refused(SignInRefusal::IdTokenRejected(rejection))
}

fn requested_scopes(scopes: &[&str]) -> Vec<oauth2::Scope> {
    iter::once(OPENID_SCOPE)
        .chain(scopes.iter().copied())
        .collect::<BTreeSet<&str>>()
        .into_iter()
        .map(|scope| oauth2::Scope::new(scope.to_string()))
        .collect()
}

pub struct SignInFlow {
    callback: RedirectUrl,
    requested_scopes: Vec<oauth2::Scope>,
    roller: Arc<JwksRoller>,
    server: Arc<AuthorizationServerClient>,
    transaction_cookie: TransactionCookie,
    transaction_issuance: TokenIssuance,
}

impl SignInFlow {
    /// # Errors
    ///
    /// Returns `SignInFlowError::MalformedCallback` when the callback is not a url.
    pub fn create(
        server: Arc<AuthorizationServerClient>,
        roller: Arc<JwksRoller>,
        callback: String,
        scopes: &[&str],
    ) -> Result<Self, SignInFlowError> {
        let transaction_issuance = TokenIssuance {
            audience: server.client_id,
            issuer: server.trusted_issuer.trust.issuer,
        };

        RedirectUrl::new(callback.clone())
            .map(|callback| Self {
                callback,
                requested_scopes: requested_scopes(scopes),
                roller,
                transaction_cookie: TransactionCookie::of(&transaction_issuance),
                server,
                transaction_issuance,
            })
            .map_err(|source| SignInFlowError::MalformedCallback { callback, source })
    }

    pub async fn begin(&self) -> SignInBeginning {
        let code_verifier = CodeVerifier::generate();
        let code_challenge = CodeChallenge::of(&code_verifier);
        let nonce = CsrfToken::new(random_token());
        let state = CsrfToken::new(random_token());
        let transaction = SignInTransactionClaims {
            code_verifier,
            nonce: nonce.secret().clone(),
            state: state.secret().clone(),
        };

        match self
            .server
            .authorization_url(AuthorizationRequest {
                code_challenge,
                nonce: nonce.into_secret(),
                redirect_uri: self.callback.clone(),
                scopes: self.requested_scopes.clone(),
                state,
            })
            .await
        {
            AuthorizationUrl::Built(authorization_url) => SignInBeginning::Redirected(
                Redirect::see_other(authorization_url.to_string())
                    .into_response()
                    .set_cookie(
                        &self
                            .transaction_cookie
                            .holding(self.sealed_transaction(&transaction)),
                    ),
            ),
            AuthorizationUrl::Unavailable(unavailability) => {
                SignInBeginning::Unavailable(unavailability)
            }
        }
    }

    pub async fn complete<TIdClaims: DeserializeOwned>(
        &self,
        request: &Request,
    ) -> SignInCompletion<TIdClaims> {
        let now = Utc::now();
        let Some(presented) = request.inputs.cookies.get(&self.transaction_cookie.name) else {
            return SignInCompletion::Refused(SignInRefusal::TransactionMissing);
        };
        let transaction = match self.verified_transaction(presented, NumericDate::from(now)) {
            JwtVerification::Rejected(rejection) => {
                return SignInCompletion::Refused(SignInRefusal::TransactionRejected(rejection));
            }
            JwtVerification::Verified(verified) => verified.claims,
        };
        let code = match callback_code(
            &request.inputs.query,
            &transaction,
            self.server.trusted_issuer.trust.issuer,
            &self.server.metadata,
        ) {
            ControlFlow::Break(completion) => return completion,
            ControlFlow::Continue(code) => code,
        };

        match self
            .server
            .exchange_authorization_code::<IdTokenFields>(
                code,
                &transaction.code_verifier,
                self.callback.clone(),
            )
            .await
        {
            EndpointOutcome::Answered(response) => {
                match self
                    .verified_id_token(&response.extra_fields().id_token, NumericDate::from(now))
                    .await
                {
                    ControlFlow::Break(completion) => completion,
                    ControlFlow::Continue(verified) => self.signed_in(
                        verified,
                        &transaction.nonce,
                        response.access_token().clone(),
                    ),
                }
            }
            EndpointOutcome::Refused(refusal) => {
                SignInCompletion::Refused(SignInRefusal::TokenRefused(refusal))
            }
            EndpointOutcome::Unavailable(unavailability) => {
                SignInCompletion::Unavailable(unavailability)
            }
        }
    }

    pub async fn userinfo<TUserinfo: DeserializeOwned, TIdClaims>(
        &self,
        signed_in: &SignedIn<TIdClaims>,
    ) -> UserinfoFetch<TUserinfo> {
        match self
            .server
            .userinfo::<UserinfoClaims<TUserinfo>>(&signed_in.access_token)
            .await
        {
            UserinfoOutcome::Answered(UserinfoClaims { sub, claims })
                if sub == signed_in.subject =>
            {
                UserinfoFetch::Fetched(claims)
            }
            UserinfoOutcome::Answered(UserinfoClaims { sub, .. }) => {
                UserinfoFetch::SubjectMismatch { found: sub }
            }
            UserinfoOutcome::Refused { status } => UserinfoFetch::Refused { status },
            UserinfoOutcome::Unavailable(unavailability) => {
                UserinfoFetch::Unavailable(unavailability)
            }
        }
    }

    fn sealed_transaction(&self, transaction: &SignInTransactionClaims) -> String {
        let registered = self
            .transaction_issuance
            .registered_claims(Utc::now(), SIGN_IN_TRANSACTION_LIFETIME_SECS);

        self.roller.jwks_secret_holder().get().current().sign_json(
            &transaction.to_payload(&registered),
            JwtType::SignInTransaction,
        )
    }

    fn signed_in<TIdClaims>(
        &self,
        VerifiedJwt {
            claims:
                IdTokenClaims {
                    azp,
                    claims,
                    nonce,
                    sub,
                },
            registered,
            ..
        }: VerifiedJwt<IdTokenClaims<TIdClaims>, IdTokenProfile>,
        transaction_nonce: &str,
        access_token: oauth2::AccessToken,
    ) -> SignInCompletion<TIdClaims> {
        if let AudienceClaim::Multiple(audiences) = &registered.aud
            && audiences.len() != 1
        {
            return SignInCompletion::Refused(SignInRefusal::AudienceNotExclusive {
                found: registered.aud,
            });
        }

        if let Some(found) = azp
            && found != self.server.client_id
        {
            return SignInCompletion::Refused(SignInRefusal::AuthorizedPartyMismatch { found });
        }

        if !equal_in_constant_time(nonce.as_bytes(), transaction_nonce.as_bytes()) {
            return SignInCompletion::Refused(SignInRefusal::NonceMismatch);
        }

        SignInCompletion::SignedIn(SignedIn {
            access_token,
            claims,
            subject: sub,
            transaction_removal: self.transaction_cookie.removal(),
        })
    }

    fn verified_transaction(
        &self,
        presented: &str,
        now: NumericDate,
    ) -> JwtVerification<SignInTransactionClaims, SignInTransactionProfile> {
        verify_serialized_jwt(
            self.roller.jwks_secret_holder().get().key_set(),
            presented,
            &self.transaction_issuance.expectation(),
            now,
        )
    }

    async fn verified_id_token<TIdClaims: DeserializeOwned>(
        &self,
        id_token: &str,
        now: NumericDate,
    ) -> ControlFlow<
        SignInCompletion<TIdClaims>,
        VerifiedJwt<IdTokenClaims<TIdClaims>, IdTokenProfile>,
    > {
        let trusted_issuer = &self.server.trusted_issuer;
        let attributed = match attribute_serialized_jwt(
            id_token,
            &JwtExpectation {
                audience: ExpectedAudience::One(self.server.client_id),
                issuer: trusted_issuer.trust.issuer,
            },
        ) {
            ControlFlow::Continue(attributed) => attributed,
            ControlFlow::Break(rejection) => {
                return ControlFlow::Break(rejected_id_token(rejection));
            }
        };

        match trusted_issuer.verify(&attributed, now).await {
            IssuerVerification::KeysAwaited => {
                ControlFlow::Break(SignInCompletion::SigningKeysAwaited)
            }
            IssuerVerification::Rejected(rejection) => {
                ControlFlow::Break(rejected_id_token(rejection))
            }
            IssuerVerification::Verified(verified) => ControlFlow::Continue(verified),
        }
    }
}
