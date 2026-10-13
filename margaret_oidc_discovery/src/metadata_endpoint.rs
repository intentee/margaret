use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MetadataEndpoint {
    AuthorizationEndpoint,
    IntrospectionEndpoint,
    JwksUri,
    TokenEndpoint,
    UserinfoEndpoint,
}

impl MetadataEndpoint {
    #[must_use]
    pub fn member_name(self) -> &'static str {
        match self {
            Self::AuthorizationEndpoint => "authorization_endpoint",
            Self::IntrospectionEndpoint => "introspection_endpoint",
            Self::JwksUri => "jwks_uri",
            Self::TokenEndpoint => "token_endpoint",
            Self::UserinfoEndpoint => "userinfo_endpoint",
        }
    }
}

impl Display for MetadataEndpoint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(self.member_name())
    }
}
