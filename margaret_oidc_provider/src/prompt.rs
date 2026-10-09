#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Prompt {
    Consent,
    Interactive,
    Login,
    LoginAndConsent,
    NoInteraction,
}

impl Prompt {
    pub(crate) fn after_login(self) -> Option<&'static str> {
        match self {
            Self::Consent | Self::LoginAndConsent => Some("consent"),
            Self::Interactive | Self::Login | Self::NoInteraction => None,
        }
    }

    pub(crate) fn forces_consent(self) -> bool {
        matches!(self, Self::Consent | Self::LoginAndConsent)
    }

    pub(crate) fn forces_login(self) -> bool {
        matches!(self, Self::Login | Self::LoginAndConsent)
    }
}
