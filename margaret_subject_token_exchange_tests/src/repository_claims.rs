use serde::Deserialize;

#[derive(Deserialize)]
pub struct RepositoryClaims {
    pub repository: String,
}
