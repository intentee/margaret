use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct UserinfoClaims<TUserinfo> {
    pub(crate) sub: String,
    #[serde(flatten)]
    pub(crate) claims: TUserinfo,
}
