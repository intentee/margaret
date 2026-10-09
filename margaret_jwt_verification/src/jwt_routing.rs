use margaret_registered_claims::registered_claims::RegisteredClaims;

use crate::attributed_jwt::AttributedJwt;

pub enum JwtRouting<'token, 'addressees, TAddressee> {
    Ambiguous(RegisteredClaims),
    Misaddressed(RegisteredClaims),
    Routed {
        addressee: &'addressees TAddressee,
        jwt: AttributedJwt<'token>,
    },
    UntrustedIssuer(RegisteredClaims),
}
