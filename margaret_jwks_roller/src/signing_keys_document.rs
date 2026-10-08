use zeroize::Zeroizing;

#[derive(Clone)]
pub struct SigningKeysDocument {
    json: Zeroizing<String>,
}

impl SigningKeysDocument {
    #[must_use]
    pub fn new(json: Zeroizing<String>) -> Self {
        Self { json }
    }

    #[must_use]
    pub fn json(&self) -> &str {
        &self.json
    }
}
