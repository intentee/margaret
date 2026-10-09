use crate::expected_audience::ExpectedAudience;

#[derive(Clone, Copy)]
pub struct JwtExpectation<'expectation> {
    pub audience: ExpectedAudience<'expectation>,
    pub issuer: &'expectation str,
}
