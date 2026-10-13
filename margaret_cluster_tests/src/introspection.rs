use serde::Deserialize;

#[derive(Deserialize)]
pub struct Introspection {
    pub active: bool,
}
