use margaret_jwt_verification::attributed_jwt::AttributedJwt;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

pub struct AttributedBearerToken<'request, 'trusted> {
    pub(crate) jwt: AttributedJwt<'request>,
    pub(crate) presented_at: NumericDate,
    pub(crate) trusted_issuer: &'trusted TrustedIssuer,
}
