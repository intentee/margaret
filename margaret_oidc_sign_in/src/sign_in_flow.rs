use std::collections::BTreeSet;
use std::iter;
use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::Utc;
use oauth2::CsrfToken;
use oauth2::PkceCodeChallenge;
use oauth2::PkceCodeVerifier;
use oauth2::RedirectUrl;
use oauth2::TokenResponse;
use serde::de::DeserializeOwned;

use margaret_authorization_server_client::authorization_request::AuthorizationRequest;
use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::authorization_url::AuthorizationUrl;
use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::userinfo_outcome::UserinfoOutcome;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_identity_session::sign_in_transaction_claims::SignInTransactionClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification::attribute_serialized_jwt::attribute_serialized_jwt;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_jwt_verification::jwt_profiling::JwtProfiling;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_trusted_issuer::issuer_verification::IssuerVerification;

use crate::callback_code::callback_code;
use crate::equal_in_constant_time::equal_in_constant_time;
use crate::id_token_claims::IdTokenClaims;
use crate::id_token_fields::IdTokenFields;
use crate::openid_scope::OPENID_SCOPE;
use crate::sign_in_beginning::SignInBeginning;
use crate::sign_in_completion::SignInCompletion;
use crate::sign_in_error::SignInError;
use crate::sign_in_refusal::SignInRefusal;
use crate::sign_in_request::SignInRequest;
use crate::signed_in::SignedIn;
use crate::transaction_cookie::TransactionCookie;
use crate::userinfo_claims::UserinfoClaims;
use crate::userinfo_fetch::UserinfoFetch;

fn rejected_id_token<TIdClaims>(rejection: JwtRejection) -> SignInCompletion<TIdClaims> {
    SignInCompletion::Refused(SignInRefusal::IdTokenRejected(rejection))
}

fn requested_scopes(scopes: &BTreeSet<Scope>) -> Vec<oauth2::Scope> {
    iter::once(OPENID_SCOPE)
        .chain(scopes.iter().map(Scope::as_str))
        .collect::<BTreeSet<&str>>()
        .into_iter()
        .map(|scope| oauth2::Scope::new(scope.to_string()))
        .collect()
}

pub struct SignInFlow {
    secret_store: Arc<JwksSecretStore>,
    server: Arc<AuthorizationServerClient>,
    transaction_cookie: TransactionCookie,
}

impl SignInFlow {
    #[must_use]
    pub fn create(
        server: Arc<AuthorizationServerClient>,
        secret_store: Arc<JwksSecretStore>,
    ) -> Self {
        let transaction_cookie = TransactionCookie::of(
            &server.trusted_issuer.trust.token_trust().issuer,
            &server.declaration.oauth_client().client_id,
        );

        Self {
            secret_store,
            server,
            transaction_cookie,
        }
    }

    /// # Errors
    ///
    /// Returns `SignInError::TransactionSecret` when the sign-in transaction cannot be signed.
    pub async fn begin(
        &self,
        SignInRequest { callback, scopes }: SignInRequest,
    ) -> Result<SignInBeginning, SignInError> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let nonce = CsrfToken::new_random();
        let state = CsrfToken::new_random();
        let transaction = SignInTransactionClaims {
            callback: callback.clone(),
            nonce: nonce.secret().clone(),
            pkce_verifier: pkce_verifier.into_secret(),
            state: state.secret().clone(),
        };

        match self
            .server
            .authorization_url(AuthorizationRequest {
                nonce: nonce.into_secret(),
                pkce_challenge,
                redirect_uri: RedirectUrl::from_url(callback),
                scopes: requested_scopes(&scopes),
                state,
            })
            .await
        {
            AuthorizationUrl::Built(authorization_url) => self
                .secret_store
                .issue_sign_in_transaction(&transaction, Utc::now())
                .map(|signed| {
                    SignInBeginning::Redirected(
                        Response::text(303, "")
                            .header("location", authorization_url.as_str())
                            .set_cookie(&self.transaction_cookie.holding(signed)),
                    )
                })
                .map_err(SignInError::TransactionSecret),
            AuthorizationUrl::Unavailable(unavailability) => {
                Ok(SignInBeginning::Unavailable(unavailability))
            }
        }
    }

    /// # Errors
    ///
    /// Returns `SignInError::TransactionSecret` when the sign-in transaction cannot be verified,
    /// and `SignInError::ClientAuthentication` when the client assertion of a `private_key_jwt`
    /// client cannot be signed.
    pub async fn complete<TIdClaims: DeserializeOwned>(
        &self,
        request: &Request,
    ) -> Result<SignInCompletion<TIdClaims>, SignInError> {
        let now = Utc::now();
        let Some(presented) = request.inputs.cookies.get(&self.transaction_cookie.name) else {
            return Ok(SignInCompletion::Refused(SignInRefusal::TransactionMissing));
        };
        let transaction = match self
            .secret_store
            .verify_sign_in_transaction(presented, now)
            .map_err(SignInError::TransactionSecret)?
        {
            JwtVerification::Rejected(rejection) => {
                return Ok(SignInCompletion::Refused(
                    SignInRefusal::TransactionRejected(rejection),
                ));
            }
            JwtVerification::Verified(verified) => verified.claims,
        };
        let code = match callback_code(
            &request.inputs.query,
            &transaction,
            &self.server.trusted_issuer.trust.token_trust().issuer,
            &self.server.metadata,
        ) {
            ControlFlow::Break(completion) => return Ok(completion),
            ControlFlow::Continue(code) => code,
        };

        Ok(
            match self
                .server
                .exchange_authorization_code::<IdTokenFields>(
                    code,
                    PkceCodeVerifier::new(transaction.pkce_verifier),
                    RedirectUrl::from_url(transaction.callback),
                )
                .await
                .map_err(SignInError::ClientAuthentication)?
            {
                EndpointOutcome::Answered(response) => {
                    match self
                        .verified_id_token(
                            &response.extra_fields().id_token,
                            NumericDate::from(now),
                        )
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
            },
        )
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
        let client_id = &self.server.declaration.oauth_client().client_id;

        if let AudienceClaim::Multiple(audiences) = &registered.aud
            && audiences.len() != 1
        {
            return SignInCompletion::Refused(SignInRefusal::AudienceNotExclusive {
                found: registered.aud,
            });
        }

        if let Some(found) = azp
            && found != client_id.as_str()
        {
            return SignInCompletion::Refused(SignInRefusal::AuthorizedPartyMismatch { found });
        }

        if !equal_in_constant_time(&nonce, transaction_nonce) {
            return SignInCompletion::Refused(SignInRefusal::NonceMismatch);
        }

        SignInCompletion::SignedIn(SignedIn {
            access_token,
            claims,
            subject: sub,
            transaction_removal: self.transaction_cookie.removal(),
        })
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
        let attributed =
            match attribute_serialized_jwt(id_token, &trusted_issuer.trust.token_trust().issuer) {
                ControlFlow::Continue(attributed) => attributed,
                ControlFlow::Break(rejection) => {
                    return ControlFlow::Break(rejected_id_token(rejection));
                }
            };
        let profiled = match attributed.profile::<IdTokenProfile>() {
            JwtProfiling::Profiled(profiled) => profiled,
            JwtProfiling::Rejected(rejection) => {
                return ControlFlow::Break(rejected_id_token(JwtRejection::Type(rejection)));
            }
        };

        match trusted_issuer
            .verify(
                &profiled,
                self.server.declaration.oauth_client().client_id.audience(),
                now,
            )
            .await
        {
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
