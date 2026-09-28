use crate::bearer_token::BearerToken;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequestAuthorization {
    Absent,
    Bearer(BearerToken),
    Malformed,
    OtherScheme,
}

impl RequestAuthorization {
    pub(crate) fn parse(field_value: Option<&str>) -> Self {
        match field_value {
            None => Self::Absent,
            Some(credentials) => match credentials.split_once(SEPARATOR) {
                None => Self::scheme_only(credentials),
                Some((scheme, parameters)) => Self::scheme_with_parameters(scheme, parameters),
            },
        }
    }

    fn scheme_only(scheme: &str) -> Self {
        if is_token(scheme) && !scheme.eq_ignore_ascii_case(BEARER_SCHEME) {
            Self::OtherScheme
        } else {
            Self::Malformed
        }
    }

    fn scheme_with_parameters(scheme: &str, parameters: &str) -> Self {
        if !is_token(scheme) {
            return Self::Malformed;
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
    use super::RequestAuthorization;
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
            RequestAuthorization::parse(Some("Basic dXNlcjpwYXNz")),
            RequestAuthorization::OtherScheme
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
