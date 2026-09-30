use crate::jwt_rejection::JwtRejection;
use crate::presented_jwt::PresentedJwt;

pub enum JwtPresentation<'token> {
    Presented(PresentedJwt<'token>),
    Rejected(JwtRejection),
}
