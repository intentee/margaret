use crate::introspected_token::IntrospectedToken;
use crate::introspection_rejection::IntrospectionRejection;

pub(crate) enum IntrospectionVerdict<TClaims> {
    Accepted(IntrospectedToken<TClaims>),
    Rejected(IntrospectionRejection),
}
