use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClientRefusal {
    ConflictingClientIds,
    MalformedCredentials,
    MissingCredentials,
    SecretRequired,
    UnexpectedSecret,
    UnknownClient,
    WrongSecret,
}

impl Display for ClientRefusal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(match self {
            Self::ConflictingClientIds => {
                "the basic credentials and the form name different clients"
            }
            Self::MalformedCredentials => "the client credentials are malformed",
            Self::MissingCredentials => "the request carries no client credentials",
            Self::SecretRequired => "the confidential client presented no secret",
            Self::UnexpectedSecret => "the public client presented a secret",
            Self::UnknownClient => "the client is not accepted",
            Self::WrongSecret => "the client secret does not match",
        })
    }
}
