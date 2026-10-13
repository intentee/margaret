use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct IdTokenClaims<TIdClaims> {
    pub(crate) azp: Option<String>,
    pub(crate) nonce: String,
    pub(crate) sub: String,
    #[serde(flatten)]
    pub(crate) claims: TIdClaims,
}
