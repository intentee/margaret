use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct PublishedRsaMembers {
    pub(crate) e: String,
    pub(crate) n: String,
}
