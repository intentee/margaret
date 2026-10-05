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
    pub(crate) fn admits(&self, found: &AudienceClaim) -> ControlFlow<ClaimsRejection> {
        match self {
            Self::AnyOf(expected) if !found.contains_any(expected) => {
                ControlFlow::Break(ClaimsRejection::AudienceOutside {
                    expected: (*expected).clone(),
                    found: found.clone(),
                })
            }
            Self::One(expected) if !found.contains(expected) => {
                ControlFlow::Break(ClaimsRejection::AudienceMismatch {
                    expected: (*expected).clone(),
                    found: found.clone(),
                })
            }
            Self::AnyOf(_) | Self::One(_) => ControlFlow::Continue(()),
        }
    }
}
