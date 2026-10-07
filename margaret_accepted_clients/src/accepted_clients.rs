use std::collections::HashMap;
use std::ops::ControlFlow;
use std::sync::Arc;

use margaret_http::request_authorization::RequestAuthorization;
use margaret_issuer_key_set::issuer_verification::IssuerVerification;
use margaret_jwt_verification::client_assertion_profile::ClientAssertionProfile;
use margaret_jwt_verification::expected_audience::ExpectedAudience;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_presentation::JwtPresentation;
use margaret_jwt_verification::presented_jwt::PresentedJwt;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_oauth_vocabulary::jwt_bearer_client_assertion_type::JWT_BEARER_CLIENT_ASSERTION_TYPE;
use margaret_provider_state_storage::assertion_spending::AssertionSpending;
use margaret_provider_state_storage::provider_state_error::ProviderStateError;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::assertion_subject::AssertionSubject;
use crate::client_assertion_max_lifetime::CLIENT_ASSERTION_MAX_LIFETIME;
use crate::client_authentication_outcome::ClientAuthenticationOutcome;
use crate::client_authentication_parameters::ClientAuthenticationParameters;
use crate::client_refusal::ClientRefusal;
use crate::registered_client::RegisteredClient;

pub struct AcceptedClients {
    clients: HashMap<&'static str, Arc<RegisteredClient>>,
    issuer: &'static str,
}

impl AcceptedClients {
    #[must_use]
    pub fn create(clients: Vec<Arc<RegisteredClient>>, issuance: TokenIssuance) -> Self {
        Self {
            clients: clients
                .into_iter()
                .map(|registered| (registered.client().client_id, registered))
                .collect(),
            issuer: issuance.issuer,
        }
    }

    /// # Errors
    ///
    /// Returns `ProviderStateError` when the provider state cannot spend the client assertion.
    pub async fn authenticate(
        &self,
        authorization: &RequestAuthorization,
        ClientAuthenticationParameters {
            client_assertion,
            client_assertion_type,
            client_id,
        }: &ClientAuthenticationParameters,
        state: &dyn StoresProviderState,
        now: NumericDate,
    ) -> Result<ClientAuthenticationOutcome<'_>, ProviderStateError> {
        if !matches!(authorization, RequestAuthorization::Absent) {
            return Ok(ClientAuthenticationOutcome::Refused(
                ClientRefusal::HeaderAuthentication,
            ));
        }

        match (client_assertion, client_assertion_type) {
            (None, None) => Ok(self.public_client(client_id.as_deref())),
            (Some(_), None) => Ok(ClientAuthenticationOutcome::Refused(
                ClientRefusal::AssertionTypeMissing,
            )),
            (None, Some(_)) => Ok(ClientAuthenticationOutcome::Refused(
                ClientRefusal::AssertionMissing,
            )),
            (Some(_), Some(assertion_type))
                if assertion_type != JWT_BEARER_CLIENT_ASSERTION_TYPE =>
            {
                Ok(ClientAuthenticationOutcome::Refused(
                    ClientRefusal::UnsupportedAssertionType,
                ))
            }
            (Some(assertion), Some(_)) => {
                self.asserted_client(assertion, client_id.as_deref(), state, now)
                    .await
            }
        }
    }

    #[must_use]
    pub fn find(&self, client_id: &str) -> Option<&RegisteredClient> {
        self.clients.get(client_id).map(AsRef::as_ref)
    }

    async fn asserted_client(
        &self,
        assertion: &str,
        form_client_id: Option<&str>,
        state: &dyn StoresProviderState,
        now: NumericDate,
    ) -> Result<ClientAuthenticationOutcome<'_>, ProviderStateError> {
        let presented = match PresentedJwt::present(assertion) {
            JwtPresentation::Presented(presented) => presented,
            JwtPresentation::Rejected(rejection) => {
                return Ok(ClientAuthenticationOutcome::Refused(
                    ClientRefusal::AssertionRejected(rejection),
                ));
            }
        };

        if form_client_id.is_some_and(|form_client_id| form_client_id != presented.claimed_issuer())
        {
            return Ok(ClientAuthenticationOutcome::Refused(
                ClientRefusal::ConflictingClientIds,
            ));
        }

        let Some(registered) = self.find(presented.claimed_issuer()) else {
            return Ok(ClientAuthenticationOutcome::Refused(
                ClientRefusal::UnknownClient,
            ));
        };
        let RegisteredClient::Confidential {
            client, key_set, ..
        } = registered
        else {
            return Ok(ClientAuthenticationOutcome::Refused(
                ClientRefusal::PublicClientAssertion,
            ));
        };
        let attributed = match presented.attribute_to(&JwtExpectation {
            audience: ExpectedAudience::Sole(self.issuer),
            issuer: client.client_id,
        }) {
            ControlFlow::Continue(attributed) => attributed,
            ControlFlow::Break(rejection) => {
                return Ok(ClientAuthenticationOutcome::Refused(
                    ClientRefusal::AssertionRejected(rejection),
                ));
            }
        };
        let VerifiedJwt {
            claims: AssertionSubject { sub },
            registered: claims,
            ..
        } = match key_set
            .verify::<AssertionSubject, ClientAssertionProfile>(&attributed, now)
            .await
        {
            IssuerVerification::KeysAwaited => {
                return Ok(ClientAuthenticationOutcome::KeysAwaited);
            }
            IssuerVerification::Rejected(rejection) => {
                return Ok(ClientAuthenticationOutcome::Refused(
                    ClientRefusal::AssertionRejected(rejection),
                ));
            }
            IssuerVerification::Verified(verified) => verified,
        };

        if sub != client.client_id {
            return Ok(ClientAuthenticationOutcome::Refused(
                ClientRefusal::SubjectMismatch { found: sub },
            ));
        }

        let limit = now.after(CLIENT_ASSERTION_MAX_LIFETIME);

        if claims.exp > limit {
            return Ok(ClientAuthenticationOutcome::Refused(
                ClientRefusal::AssertionOutlivesLimit {
                    exp: claims.exp,
                    limit,
                },
            ));
        }

        let Some(jti) = claims.jti else {
            return Ok(ClientAuthenticationOutcome::Refused(
                ClientRefusal::AssertionIdentifierMissing,
            ));
        };

        match state
            .spend_client_assertion(client.client_id, TokenDigest::of(&jti), claims.exp, now)
            .await?
        {
            AssertionSpending::Refused(refusal) => Ok(ClientAuthenticationOutcome::Refused(
                ClientRefusal::AssertionRefused(refusal),
            )),
            AssertionSpending::Spent => Ok(ClientAuthenticationOutcome::Authenticated(registered)),
        }
    }

    fn public_client(&self, client_id: Option<&str>) -> ClientAuthenticationOutcome<'_> {
        let Some(client_id) = client_id else {
            return ClientAuthenticationOutcome::Refused(ClientRefusal::MissingCredentials);
        };

        match self.find(client_id) {
            Some(registered @ RegisteredClient::Public(_)) => {
                ClientAuthenticationOutcome::Authenticated(registered)
            }
            Some(RegisteredClient::Confidential { .. }) => {
                ClientAuthenticationOutcome::Refused(ClientRefusal::AssertionRequired)
            }
            None => ClientAuthenticationOutcome::Refused(ClientRefusal::UnknownClient),
        }
    }
}
