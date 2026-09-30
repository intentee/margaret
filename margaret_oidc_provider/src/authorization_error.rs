#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthorizationError {
    AccessDenied,
    ConsentRequired,
    InvalidRequest,
    InvalidScope,
    LoginRequired,
    UnauthorizedClient,
    UnsupportedResponseType,
}

impl AuthorizationError {
    pub(crate) fn wire_name(self) -> &'static str {
        match self {
            Self::AccessDenied => "access_denied",
            Self::ConsentRequired => "consent_required",
            Self::InvalidRequest => "invalid_request",
            Self::InvalidScope => "invalid_scope",
            Self::LoginRequired => "login_required",
            Self::UnauthorizedClient => "unauthorized_client",
            Self::UnsupportedResponseType => "unsupported_response_type",
        }
    }
}
