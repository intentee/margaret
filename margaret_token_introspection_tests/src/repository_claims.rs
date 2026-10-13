use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RepositoryClaims {
    pub repository: String,
}
