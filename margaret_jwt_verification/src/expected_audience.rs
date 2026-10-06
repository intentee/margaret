use std::collections::BTreeSet;
use std::ops::ControlFlow;

use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::audience_claim::AudienceClaim;

use crate::claims_rejection::ClaimsRejection;

pub enum ExpectedAudience<'expectation> {
    AnyOf(&'expectation BTreeSet<Audience>),
    One(&'expectation Audience),
}

impl ExpectedAudience<'_> {
    pub(crate) fn admits(&self, found: &AudienceClaim) -> bool {
        match self {
            Self::AnyOf(expected) => found.contains_any(expected),
            Self::One(expected) => found.contains(expected),
        }
    }

    pub(crate) fn check(&self, found: &AudienceClaim) -> ControlFlow<ClaimsRejection> {
        if self.admits(found) {
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(match self {
            Self::AnyOf(expected) => ClaimsRejection::AudienceOutside {
                expected: (*expected).clone(),
                found: found.clone(),
            },
            Self::One(expected) => ClaimsRejection::AudienceMismatch {
                expected: (*expected).clone(),
                found: found.clone(),
            },
        })
    }
}
