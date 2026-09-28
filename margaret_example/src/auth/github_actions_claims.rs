use serde::Deserialize;

#[derive(Deserialize)]
pub struct GithubActionsClaims {
    pub repository: String,
    pub repository_id: String,
    pub repository_owner_id: String,
}
