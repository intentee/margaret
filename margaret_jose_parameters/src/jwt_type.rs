use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JwtType {
    AccessToken,
    ClientAuthentication,
    Jwt,
    SignInTransaction,
}

impl JwtType {
    pub const ALL: [Self; 4] = [
        Self::AccessToken,
        Self::ClientAuthentication,
        Self::Jwt,
        Self::SignInTransaction,
    ];

    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::AccessToken => "at+jwt",
            Self::ClientAuthentication => "client-authentication+jwt",
            Self::Jwt => "JWT",
            Self::SignInTransaction => "margaret-sign-in+jwt",
        }
    }
}

impl Display for JwtType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(self.wire_name())
    }
}

#[cfg(test)]
mod tests {
    use super::JwtType;

    #[test]
    fn displays_the_wire_name() {
        assert_eq!(JwtType::AccessToken.to_string(), "at+jwt");
    }
}
