#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthorizationError {
    AccessDenied,
    ConflictingPromptValues,
    ConsentRequired,
    LoginRequired,
    MalformedCodeChallenge,
    MalformedMaxAge,
    MalformedScope,
    MissingCodeChallenge,
    MissingCodeChallengeMethod,
    ScopeNotGranted,
    UnsupportedCodeChallengeMethod,
    UnsupportedPromptValue,
    UnsupportedResponseType,
}

impl AuthorizationError {
    pub(crate) fn description(self) -> &'static str {
        match self {
            Self::AccessDenied => "the end-user denied the authorization",
            Self::ConflictingPromptValues => "the prompt value none cannot be combined with others",
            Self::ConsentRequired => "the end-user has to consent, which needs an interaction",
            Self::LoginRequired => "the end-user has to authenticate, which needs an interaction",
            Self::MalformedCodeChallenge => {
                "the code challenge is not a base64url encoded sha-256 digest"
            }
            Self::MalformedMaxAge => "the max_age parameter is not a number of seconds",
            Self::MalformedScope => "the scope parameter is not a list of scope tokens",
            Self::MissingCodeChallenge => "the request carries no code challenge",
            Self::MissingCodeChallengeMethod => {
                "the code challenge method defaults to plain, which is not supported"
            }
            Self::ScopeNotGranted => "the scope includes a scope the client may not request",
            Self::UnsupportedCodeChallengeMethod => "the code challenge method has to be S256",
            Self::UnsupportedPromptValue => {
                "the prompt parameter lists a value that is not supported"
            }
            Self::UnsupportedResponseType => "the response type has to be code",
        }
    }

    pub(crate) fn wire_name(self) -> &'static str {
        match self {
            Self::AccessDenied => "access_denied",
            Self::ConflictingPromptValues
            | Self::MalformedCodeChallenge
            | Self::MalformedMaxAge
            | Self::MissingCodeChallenge
            | Self::MissingCodeChallengeMethod
            | Self::UnsupportedCodeChallengeMethod
            | Self::UnsupportedPromptValue => "invalid_request",
            Self::ConsentRequired => "consent_required",
            Self::LoginRequired => "login_required",
            Self::MalformedScope | Self::ScopeNotGranted => "invalid_scope",
            Self::UnsupportedResponseType => "unsupported_response_type",
        }
    }
}
