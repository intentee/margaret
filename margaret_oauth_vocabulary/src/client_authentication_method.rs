use std::str::FromStr;

use crate::oauth_vocabulary_error::OAuthVocabularyError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClientAuthenticationMethod {
    ClientSecretBasic,
    None,
    PrivateKeyJwt,
}

impl ClientAuthenticationMethod {
    pub const ALL: [Self; 3] = [Self::ClientSecretBasic, Self::None, Self::PrivateKeyJwt];

    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::ClientSecretBasic => "client_secret_basic",
            Self::None => "none",
            Self::PrivateKeyJwt => "private_key_jwt",
        }
    }
}

impl FromStr for ClientAuthenticationMethod {
    type Err = OAuthVocabularyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|method| method.wire_name() == value)
            .ok_or_else(
                || OAuthVocabularyError::UnsupportedClientAuthenticationMethod {
                    value: value.to_string(),
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use super::ClientAuthenticationMethod;

    #[test]
    fn reads_every_method_by_its_wire_name() {
        for method in ClientAuthenticationMethod::ALL {
            assert_eq!(
                method
                    .wire_name()
                    .parse::<ClientAuthenticationMethod>()
                    .expect("the wire name is a method"),
                method
            );
        }
    }

    #[test]
    fn rejects_a_method_outside_the_vocabulary() {
        assert_eq!(
            "client_secret_jwt"
                .parse::<ClientAuthenticationMethod>()
                .expect_err("the method is not supported")
                .to_string(),
            "the client authentication method 'client_secret_jwt' is not supported"
        );
    }
}
