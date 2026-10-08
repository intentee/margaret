use serde::Deserialize;

use margaret::framework::oidc_provider::consent_decision::ConsentDecision;

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentChoice {
    Approve,
    Deny,
}

impl ConsentChoice {
    #[must_use]
    pub fn decision(self) -> ConsentDecision {
        match self {
            Self::Approve => ConsentDecision::Approved,
            Self::Deny => ConsentDecision::Denied,
        }
    }
}
