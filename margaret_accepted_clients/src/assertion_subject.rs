use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct AssertionSubject {
    pub(crate) sub: String,
}
