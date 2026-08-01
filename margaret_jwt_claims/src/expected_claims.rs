#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedClaims {
    pub audience: String,
    pub issuer: String,
}
