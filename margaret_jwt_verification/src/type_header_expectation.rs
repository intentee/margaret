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
        match (self, typ) {
            (Self::Required(expected), None) => {
                ControlFlow::Break(TypeRejection::Missing { expected })
            }
            (Self::Required(expected), Some(HeaderType::Supported(found)))
                if *found == expected =>
            {
                ControlFlow::Continue(())
            }
            (Self::Required(expected), Some(found)) => {
                ControlFlow::Break(TypeRejection::Mismatch {
                    expected,
                    found: found.clone(),
                })
            }
            (Self::UntypedOr(_), None) => ControlFlow::Continue(()),
            (Self::UntypedOr(accepted), Some(HeaderType::Supported(found)))
                if accepted.contains(found) =>
            {
                ControlFlow::Continue(())
            }
            (Self::UntypedOr(accepted), Some(found)) => {
                ControlFlow::Break(TypeRejection::Unaccepted {
                    accepted,
                    found: found.clone(),
                })
            }
        }
    }
}
