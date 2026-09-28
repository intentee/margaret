use std::ops::ControlFlow;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::header_type::HeaderType;

use crate::type_rejection::TypeRejection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeHeaderExpectation {
    Optional(JwtType),
    Required(JwtType),
}

impl TypeHeaderExpectation {
    pub(crate) fn check(self, typ: Option<&HeaderType>) -> ControlFlow<TypeRejection> {
        let (Self::Optional(expected) | Self::Required(expected)) = self;

        match typ {
            None => match self {
                Self::Optional(_) => ControlFlow::Continue(()),
                Self::Required(_) => ControlFlow::Break(TypeRejection::Missing { expected }),
            },
            Some(HeaderType::Supported(found)) if *found == expected => ControlFlow::Continue(()),
            Some(found) => ControlFlow::Break(TypeRejection::Mismatch {
                expected,
                found: found.clone(),
            }),
        }
    }
}
