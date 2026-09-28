use http::header::WWW_AUTHENTICATE;

use crate::response::Response;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BearerChallenge {
    InvalidRequest,
    InvalidToken,
    MissingCredentials,
}

impl BearerChallenge {
    #[must_use]
    pub fn response(self) -> Response {
        match self {
            Self::InvalidRequest => Response::text(400, "Bad Request").header(
                WWW_AUTHENTICATE.as_str(),
                "Bearer error=\"invalid_request\"",
            ),
            Self::InvalidToken => Response::unauthorized()
                .header(WWW_AUTHENTICATE.as_str(), "Bearer error=\"invalid_token\""),
            Self::MissingCredentials => {
                Response::unauthorized().header(WWW_AUTHENTICATE.as_str(), "Bearer")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use http::header::WWW_AUTHENTICATE;

    use super::BearerChallenge;

    fn status_and_challenge(challenge: BearerChallenge) -> String {
        let response = challenge.response().into_http();
        let header = response
            .headers()
            .get(WWW_AUTHENTICATE)
            .expect("the response carries a bearer challenge");

        format!(
            "{} {}",
            response.status().as_u16(),
            header.to_str().expect("the challenge is visible ascii")
        )
    }

    #[test]
    fn challenges_missing_credentials_without_an_error_code() {
        assert_eq!(
            status_and_challenge(BearerChallenge::MissingCredentials),
            "401 Bearer"
        );
    }

    #[test]
    fn challenges_a_malformed_request_as_an_invalid_request() {
        assert_eq!(
            status_and_challenge(BearerChallenge::InvalidRequest),
            "400 Bearer error=\"invalid_request\""
        );
    }

    #[test]
    fn challenges_a_rejected_token_as_an_invalid_token() {
        assert_eq!(
            status_and_challenge(BearerChallenge::InvalidToken),
            "401 Bearer error=\"invalid_token\""
        );
    }
}
