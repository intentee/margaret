use std::ops::ControlFlow;

use margaret_registered_claims::audience_claim::AudienceClaim;

use crate::claims_rejection::ClaimsRejection;

#[derive(Clone, Copy)]
pub enum ExpectedAudience<'expectation> {
    AnyOf(&'expectation [&'expectation str]),
    One(&'expectation str),
    Sole(&'expectation str),
}

impl ExpectedAudience<'_> {
    pub(crate) fn admits(&self, found: &AudienceClaim) -> bool {
        match self {
            Self::AnyOf(expected) => found.contains_any(expected),
            Self::One(expected) => found.contains(expected),
            Self::Sole(expected) => {
                matches!(found, AudienceClaim::Single(audience) if audience == expected)
            }
        }
    }

    pub(crate) fn check(&self, found: &AudienceClaim) -> ControlFlow<ClaimsRejection> {
        if self.admits(found) {
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(match self {
            Self::AnyOf(expected) => ClaimsRejection::AudienceOutside {
                expected: expected
                    .iter()
                    .map(|audience| (*audience).to_string())
                    .collect(),
                found: found.clone(),
            },
            Self::One(expected) => ClaimsRejection::AudienceMismatch {
                expected: (*expected).to_string(),
                found: found.clone(),
            },
            Self::Sole(expected) => ClaimsRejection::AudienceNotSole {
                expected: (*expected).to_string(),
                found: found.clone(),
            },
        })
    }
}
