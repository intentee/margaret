use crate::jwt_rejection::JwtRejection;
use crate::presented_jwt::PresentedJwt;

pub enum JwtPresentation<'token> {
    Presented(Box<PresentedJwt<'token>>),
    Rejected(JwtRejection),
}
