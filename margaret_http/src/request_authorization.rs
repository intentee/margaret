use std::iter;
use std::ops::ControlFlow;

use headers::Authorization;
use headers::Header;
use http::HeaderValue;

use crate::basic_credentials::BasicCredentials;
use crate::bearer_challenge::BearerChallenge;
use crate::bearer_token::BearerToken;

const BASIC_SCHEME: &str = "Basic";
const BEARER_SCHEME: &str = "Bearer";
const TOKEN_SYMBOLS: &[u8] = b"!#$%&'*+-.^_`|~";
const B64TOKEN_SYMBOLS: &[u8] = b"-._~+/";
const B64TOKEN_PADDING: char = '=';
const SEPARATOR: char = ' ';

fn is_token(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || TOKEN_SYMBOLS.contains(&byte))
}

fn is_b64token(value: &str) -> bool {
    let body = value.trim_end_matches(B64TOKEN_PADDING);

    !body.is_empty()
        && body
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || B64TOKEN_SYMBOLS.contains(&byte))
}

fn basic_credentials(field_value: &str) -> RequestAuthorization {
    HeaderValue::from_str(field_value)
        .ok()
        .and_then(|value| Authorization::decode(&mut iter::once(&value)).ok())
        .map_or(RequestAuthorization::Malformed, |authorization| {
            RequestAuthorization::Basic(BasicCredentials::new(authorization))
        })
}

#[derive(Debug, PartialEq)]
pub enum RequestAuthorization {
    Absent,
    Basic(BasicCredentials),
    Bearer(BearerToken),
    Malformed,
    OtherScheme,
}

impl RequestAuthorization {
    #[must_use]
    pub fn parse(field_value: Option<&str>) -> Self {
        match field_value {
            None => Self::Absent,
            Some(credentials) => match credentials.split_once(SEPARATOR) {
                None => Self::scheme_only(credentials),
                Some((scheme, parameters)) => {
                    Self::scheme_with_parameters(credentials, scheme, parameters)
                }
            },
        }
    }

    /// # Errors
    ///
    /// Breaks with the `BearerChallenge` answering a request that presents no bearer token, or a
    /// malformed one.
    pub fn bearer(&self) -> ControlFlow<BearerChallenge, &BearerToken> {
        match self {
            Self::Absent | Self::Basic(_) | Self::OtherScheme => {
                ControlFlow::Break(BearerChallenge::MissingCredentials)
            }
            Self::Bearer(token) => ControlFlow::Continue(token),
            Self::Malformed => ControlFlow::Break(BearerChallenge::InvalidRequest),
        }
    }

    fn scheme_only(scheme: &str) -> Self {
        if is_token(scheme)
            && !scheme.eq_ignore_ascii_case(BASIC_SCHEME)
            && !scheme.eq_ignore_ascii_case(BEARER_SCHEME)
        {
            Self::OtherScheme
        } else {
            Self::Malformed
        }
    }

    fn scheme_with_parameters(field_value: &str, scheme: &str, parameters: &str) -> Self {
        if !is_token(scheme) {
            return Self::Malformed;
        }

        if scheme.eq_ignore_ascii_case(BASIC_SCHEME) {
            return basic_credentials(field_value);
        }

        if !scheme.eq_ignore_ascii_case(BEARER_SCHEME) {
            return Self::OtherScheme;
        }

        let token = parameters.trim_start_matches(SEPARATOR);

        if is_b64token(token) {
            Self::Bearer(BearerToken::new(token.to_string()))
        } else {
            Self::Malformed
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::ControlFlow;

    use headers::Authorization;

    use super::RequestAuthorization;
    use crate::basic_credentials::BasicCredentials;
    use crate::bearer_challenge::BearerChallenge;
    use crate::bearer_token::BearerToken;

    fn bearer(token: &str) -> RequestAuthorization {
        RequestAuthorization::Bearer(BearerToken::new(token.to_string()))
    }

    #[test]
    fn reads_an_absent_field_as_absent() {
        assert_eq!(
            RequestAuthorization::parse(None),
            RequestAuthorization::Absent
        );
    }

    #[test]
    fn reads_a_bearer_token() {
        assert_eq!(
            RequestAuthorization::parse(Some("Bearer mF_9.B5f-4.1JqM")),
            bearer("mF_9.B5f-4.1JqM")
        );
    }

    #[test]
    fn compares_the_scheme_case_insensitively() {
        assert_eq!(
            RequestAuthorization::parse(Some("bEARER abc")),
            bearer("abc")
        );
    }

    #[test]
    fn accepts_several_spaces_before_the_token() {
        assert_eq!(
            RequestAuthorization::parse(Some("Bearer   abc")),
            bearer("abc")
        );
    }

    #[test]
    fn keeps_trailing_padding_of_the_token() {
        assert_eq!(
            RequestAuthorization::parse(Some("Bearer abc+/==")),
            bearer("abc+/==")
        );
    }

    #[test]
    fn rejects_a_bearer_scheme_without_a_token() {
        assert_eq!(
            RequestAuthorization::parse(Some("Bearer")),
            RequestAuthorization::Malformed
        );
    }

    #[test]
    fn rejects_an_empty_bearer_token() {
        assert_eq!(
            RequestAuthorization::parse(Some("Bearer ")),
            RequestAuthorization::Malformed
        );
    }

    #[test]
    fn rejects_a_token_with_a_space() {
        assert_eq!(
            RequestAuthorization::parse(Some("Bearer abc def")),
            RequestAuthorization::Malformed
        );
    }

    #[test]
    fn rejects_padding_inside_the_token() {
        assert_eq!(
            RequestAuthorization::parse(Some("Bearer ab=c")),
            RequestAuthorization::Malformed
        );
    }

    #[test]
    fn rejects_a_token_of_padding_alone() {
        assert_eq!(
            RequestAuthorization::parse(Some("Bearer ==")),
            RequestAuthorization::Malformed
        );
    }

    #[test]
    fn reads_another_scheme_with_parameters() {
        assert_eq!(
            RequestAuthorization::parse(Some("Digest username=\"user\"")),
            RequestAuthorization::OtherScheme
        );
    }

    #[test]
    fn reads_basic_credentials() {
        assert_eq!(
            RequestAuthorization::parse(Some("bASIC dXNlcjpwYTpzcw==")),
            RequestAuthorization::Basic(BasicCredentials::new(Authorization::basic(
                "user", "pa:ss"
            )))
        );
    }

    #[test]
    fn keeps_the_basic_password_out_of_its_debug_output() {
        assert_eq!(
            format!(
                "{:?}",
                RequestAuthorization::parse(Some("Basic dXNlcjpwYXNz"))
            ),
            "Basic(BasicCredentials { user_id: \"user\", .. })"
        );
    }

    #[test]
    fn rejects_basic_credentials_that_are_not_base64() {
        assert_eq!(
            RequestAuthorization::parse(Some("Basic !!!")),
            RequestAuthorization::Malformed
        );
    }

    #[test]
    fn rejects_basic_credentials_without_a_password_separator() {
        assert_eq!(
            RequestAuthorization::parse(Some("Basic dXNlcg==")),
            RequestAuthorization::Malformed
        );
    }

    #[test]
    fn rejects_a_basic_scheme_without_credentials() {
        assert_eq!(
            RequestAuthorization::parse(Some("Basic")),
            RequestAuthorization::Malformed
        );
    }

    #[test]
    fn rejects_basic_credentials_that_are_not_a_header_value() {
        assert_eq!(
            RequestAuthorization::parse(Some("Basic dXNlcjpwYXNz\n")),
            RequestAuthorization::Malformed
        );
    }

    #[test]
    fn presents_a_bearer_token() {
        assert_eq!(
            bearer("abc").bearer(),
            ControlFlow::Continue(&BearerToken::new("abc".to_string()))
        );
    }

    #[test]
    fn challenges_a_request_without_a_bearer_token_for_credentials() {
        assert_eq!(
            [
                RequestAuthorization::Absent,
                RequestAuthorization::parse(Some("Basic dXNlcjpwYXNz")),
                RequestAuthorization::OtherScheme,
            ]
            .iter()
            .map(RequestAuthorization::bearer)
            .collect::<Vec<_>>(),
            vec![ControlFlow::Break(BearerChallenge::MissingCredentials); 3]
        );
    }

    #[test]
    fn challenges_a_malformed_authorization_as_an_invalid_request() {
        assert_eq!(
            RequestAuthorization::Malformed.bearer(),
            ControlFlow::Break(BearerChallenge::InvalidRequest)
        );
    }

    #[test]
    fn reads_another_scheme_alone() {
        assert_eq!(
            RequestAuthorization::parse(Some("Negotiate")),
            RequestAuthorization::OtherScheme
        );
    }

    #[test]
    fn rejects_an_empty_field() {
        assert_eq!(
            RequestAuthorization::parse(Some("")),
            RequestAuthorization::Malformed
        );
    }

    #[test]
    fn rejects_a_scheme_that_is_not_a_token() {
        assert_eq!(
            RequestAuthorization::parse(Some("Bea:rer abc")),
            RequestAuthorization::Malformed
        );
    }
}
