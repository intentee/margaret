use crate::attributed_jwt::AttributedJwt;
use crate::presented_jwt::PresentedJwt;

pub enum JwtAttribution<'token> {
    Attributed(AttributedJwt<'token>),
    Unattributed(PresentedJwt<'token>),
}
