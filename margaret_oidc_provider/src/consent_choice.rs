use serde::Deserialize;

use crate::consent_decision::ConsentDecision;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::ConsentChoice;
    use crate::consent_decision::ConsentDecision;

    #[test]
    fn decides_as_the_end_user_chose() {
        assert_eq!(
            [ConsentChoice::Approve, ConsentChoice::Deny].map(ConsentChoice::decision),
            [ConsentDecision::Approved, ConsentDecision::Denied]
        );
    }
}
