#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenTypeHint {
    AccessToken,
}

impl TokenTypeHint {
    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::AccessToken => "access_token",
        }
    }
}
