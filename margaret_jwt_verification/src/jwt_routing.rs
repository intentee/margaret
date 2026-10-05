use crate::attributed_jwt::AttributedJwt;
use crate::presented_jwt::PresentedJwt;

pub enum JwtRouting<'token, 'addressees, TAddressee> {
    Routed {
        addressee: &'addressees TAddressee,
        jwt: AttributedJwt<'token>,
    },
    Unrouted(PresentedJwt<'token>),
}
