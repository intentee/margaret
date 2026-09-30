use oauth2::RequestTokenError;
use oauth2::StandardTokenResponse;
use oauth2::TokenResponse;
use oauth2::basic::BasicErrorResponse;
use oauth2::basic::BasicTokenType;

use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;

use crate::server_unavailability::ServerUnavailability;

pub enum EndpointOutcome<TAnswer> {
    Answered(TAnswer),
    Refused(BasicErrorResponse),
    Unavailable(ServerUnavailability),
}

impl<TAnswer> EndpointOutcome<TAnswer> {
    pub(crate) fn of(
        result: Result<TAnswer, RequestTokenError<IssuerExchangeError, BasicErrorResponse>>,
    ) -> Self {
        match result {
            Ok(answer) => Self::Answered(answer),
            Err(RequestTokenError::ServerResponse(refusal)) => Self::Refused(refusal),
            Err(RequestTokenError::Request(failure)) => {
                Self::Unavailable(ServerUnavailability::from(failure))
            }
            Err(RequestTokenError::Parse(source, _answer)) => {
                Self::Unavailable(ServerUnavailability::MalformedAnswer { source })
            }
            Err(RequestTokenError::Other(description)) => {
                Self::Unavailable(ServerUnavailability::UnexpectedAnswer { description })
            }
        }
    }
}

impl<TExtraFields: oauth2::ExtraTokenFields>
    EndpointOutcome<StandardTokenResponse<TExtraFields, BasicTokenType>>
{
    pub(crate) fn of_bearer_token(
        result: Result<
            StandardTokenResponse<TExtraFields, BasicTokenType>,
            RequestTokenError<IssuerExchangeError, BasicErrorResponse>,
        >,
    ) -> Self {
        match Self::of(result) {
            Self::Answered(response) if *response.token_type() != BasicTokenType::Bearer => {
                Self::Unavailable(ServerUnavailability::UnsupportedTokenType {
                    token_type: response.token_type().clone(),
                })
            }
            outcome => outcome,
        }
    }
}
