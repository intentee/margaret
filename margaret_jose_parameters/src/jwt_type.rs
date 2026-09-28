use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JwtType {
    AccessToken,
    Jwt,
}

impl JwtType {
    pub const ALL: [Self; 2] = [Self::AccessToken, Self::Jwt];

    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::AccessToken => "at+jwt",
            Self::Jwt => "JWT",
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
