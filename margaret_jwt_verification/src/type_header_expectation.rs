use std::ops::ControlFlow;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::header_type::HeaderType;

use crate::type_rejection::TypeRejection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeHeaderExpectation {
    Required(JwtType),
    UntypedOr(&'static [JwtType]),
}

impl TypeHeaderExpectation {
    pub(crate) fn check(self, typ: Option<&HeaderType>) -> ControlFlow<TypeRejection> {
        match self {
            Self::Required(expected) => match typ {
                None => ControlFlow::Break(TypeRejection::Missing { expected }),
                Some(HeaderType::Supported(found)) if *found == expected => {
                    ControlFlow::Continue(())
                }
                Some(found) => ControlFlow::Break(TypeRejection::Mismatch {
                    expected,
                    found: found.clone(),
                }),
            },
            Self::UntypedOr(accepted) => match typ {
                None => ControlFlow::Continue(()),
                Some(HeaderType::Supported(found)) if accepted.contains(found) => {
                    ControlFlow::Continue(())
                }
                Some(found) => ControlFlow::Break(TypeRejection::Unaccepted {
                    accepted,
                    found: found.clone(),
                }),
            },
        }
    }
}
