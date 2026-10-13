use crate::profiled_jwt::ProfiledJwt;
use crate::type_rejection::TypeRejection;

pub enum JwtProfiling<'jwt, TProfile> {
    Profiled(ProfiledJwt<'jwt, TProfile>),
    Rejected(TypeRejection),
}
