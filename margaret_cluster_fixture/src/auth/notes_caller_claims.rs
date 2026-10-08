use serde::Deserialize;

#[derive(Deserialize)]
pub struct NotesCallerClaims {
    pub sub: String,
}
