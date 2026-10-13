use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArtifactClaims {
    pub repository: String,
}
