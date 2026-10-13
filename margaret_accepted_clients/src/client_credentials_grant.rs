#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClientCredentialsGrant {
    Granted { scopes: &'static [&'static str] },
    Withheld,
}
