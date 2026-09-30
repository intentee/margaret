use margaret_http::response_continuation::ResponseContinuation;

use crate::introspected_token::IntrospectedToken;

pub enum IntrospectionAdmission<TClaims> {
    Admitted(IntrospectedToken<TClaims>),
    Refused(ResponseContinuation),
    Unaddressed,
}
