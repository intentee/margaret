use serde::Deserialize;

#[derive(Deserialize)]
pub struct Claims {
    pub repository: String,
}
