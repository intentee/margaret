#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenExchangeGrant {
    Granted { scopes: &'static [&'static str] },
    Withheld,
}
