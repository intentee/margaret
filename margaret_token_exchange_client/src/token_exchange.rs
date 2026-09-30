use std::sync::Arc;

use oauth2::StandardTokenResponse;
use oauth2::TokenResponse;
use oauth2::basic::BasicTokenType;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::authorization_server_client_error::AuthorizationServerClientError;
use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::form_parameter::FormParameter;
use margaret_authorization_server_client::token_target::TokenTarget;

use crate::exchanged_token::ExchangedToken;
use crate::issued_token_type_fields::IssuedTokenTypeFields;
use crate::issued_token_type_mismatch::IssuedTokenTypeMismatch;
use crate::subject_token::SubjectToken;
use crate::subject_token_type::SubjectTokenType;

const TOKEN_EXCHANGE_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:token-exchange";

fn exchanged(
    response: &StandardTokenResponse<IssuedTokenTypeFields, BasicTokenType>,
) -> ExchangedToken {
    match &response.extra_fields().issued_token_type {
        Some(issued_token_type) if issued_token_type == SubjectTokenType::AccessToken.urn() => {
            ExchangedToken::Exchanged(response.access_token().clone())
        }
        Some(issued_token_type) => {
            ExchangedToken::IssuedTokenTypeMismatch(IssuedTokenTypeMismatch::Other {
                issued_token_type: issued_token_type.clone(),
            })
        }
        None => ExchangedToken::IssuedTokenTypeMismatch(IssuedTokenTypeMismatch::Missing),
    }
}

pub struct TokenExchange {
    server: Arc<AuthorizationServerClient>,
}

impl TokenExchange {
    #[must_use]
    pub fn create(server: Arc<AuthorizationServerClient>) -> Self {
        Self { server }
    }

    /// # Errors
    ///
    /// Returns `AuthorizationServerClientError::AssertionSigning` when the client assertion of a
    /// `private_key_jwt` client cannot be signed.
    pub async fn exchange(
        &self,
        subject: &SubjectToken,
        target: &TokenTarget,
    ) -> Result<ExchangedToken, AuthorizationServerClientError> {
        let mut parameters = vec![
            FormParameter {
                name: "requested_token_type",
                value: SubjectTokenType::AccessToken.urn().to_string(),
            },
            FormParameter {
                name: "subject_token",
                value: subject.token.to_string(),
            },
            FormParameter {
                name: "subject_token_type",
                value: subject.token_type.urn().to_string(),
            },
        ];

        parameters.extend(target.form_parameters());

        Ok(
            match self
                .server
                .request_grant::<IssuedTokenTypeFields>(TOKEN_EXCHANGE_GRANT_TYPE, parameters)
                .await?
            {
                EndpointOutcome::Answered(response) => exchanged(&response),
                EndpointOutcome::Refused(refusal) => ExchangedToken::Refused(refusal),
                EndpointOutcome::Unavailable(unavailability) => {
                    ExchangedToken::Unavailable(unavailability)
                }
            },
        )
    }
}
