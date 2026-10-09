use http::HeaderValue;
use http::Response;
use http::StatusCode;
use http::header::CONTENT_TYPE;
use mime::Mime;
use oauth2::StandardTokenResponse;
use oauth2::TokenResponse;
use oauth2::basic::BasicErrorResponse;
use oauth2::basic::BasicTokenType;
use serde::de::DeserializeOwned;

use margaret_issuer_request::issuer_answer::IssuerAnswer;
use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;

use crate::answer_parsing::AnswerParsing;
use crate::server_unavailability::ServerUnavailability;

fn is_json(content_type: &HeaderValue) -> bool {
    content_type
        .to_str()
        .ok()
        .and_then(|content_type| content_type.parse::<Mime>().ok())
        .is_some_and(|mime| mime.type_() == mime::APPLICATION && mime.subtype() == mime::JSON)
}

pub enum EndpointOutcome<TAnswer> {
    Answered(TAnswer),
    Refused(BasicErrorResponse),
    Unavailable(ServerUnavailability),
}

impl<TAnswer: DeserializeOwned> EndpointOutcome<TAnswer> {
    pub(crate) fn of(exchanged: Result<IssuerAnswer, IssuerExchangeError>) -> Self {
        match exchanged {
            Ok(IssuerAnswer::Received(answer)) if answer.status() == StatusCode::OK => {
                Self::answered(&answer)
            }
            Ok(IssuerAnswer::Received(answer)) => Self::refused(&answer),
            Ok(IssuerAnswer::Oversized { max_bytes }) => {
                Self::Unavailable(ServerUnavailability::OversizedAnswer { max_bytes })
            }
            Err(failure) => Self::Unavailable(ServerUnavailability::Exchange(failure)),
        }
    }

    fn answered(answer: &Response<Vec<u8>>) -> Self {
        match answer.headers().get(CONTENT_TYPE) {
            Some(content_type) if !is_json(content_type) => {
                Self::Unavailable(ServerUnavailability::UnexpectedContentType {
                    content_type: content_type.clone(),
                })
            }
            _ if answer.body().is_empty() => Self::Unavailable(ServerUnavailability::EmptyAnswer),
            _ => Self::parsed_into(answer.body(), Self::Answered),
        }
    }

    fn parsed_into<TParsed: DeserializeOwned>(body: &[u8], outcome: fn(TParsed) -> Self) -> Self {
        match AnswerParsing::of(body) {
            AnswerParsing::Parsed(parsed) => outcome(parsed),
            AnswerParsing::Malformed(source) => {
                Self::Unavailable(ServerUnavailability::MalformedAnswer { source })
            }
        }
    }

    fn refused(answer: &Response<Vec<u8>>) -> Self {
        if answer.body().is_empty() {
            Self::Unavailable(ServerUnavailability::EmptyRefusal {
                status: answer.status(),
            })
        } else {
            Self::parsed_into(answer.body(), Self::Refused)
        }
    }
}

impl<TExtraFields: oauth2::ExtraTokenFields>
    EndpointOutcome<StandardTokenResponse<TExtraFields, BasicTokenType>>
{
    pub(crate) fn bearer_checked(self) -> Self {
        match self {
            Self::Answered(response) if *response.token_type() != BasicTokenType::Bearer => {
                Self::Unavailable(ServerUnavailability::UnsupportedTokenType {
                    token_type: response.token_type().clone(),
                })
            }
            outcome => outcome,
        }
    }
}
